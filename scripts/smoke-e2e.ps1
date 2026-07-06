param(
    [string]$BaseUrl = "http://127.0.0.1:3000",
    [string]$WsUrl = "ws://127.0.0.1:3000/ws",
    [string]$Usuario = "visualizador",
    [string]$Contrasena = "visualizador"
)

$ErrorActionPreference = "Stop"

function Receive-WebSocketText {
    param(
        [System.Net.WebSockets.ClientWebSocket]$Socket,
        [int]$TimeoutSeconds = 10
    )

    $buffer = [System.Array]::CreateInstance([byte], 4096)
    $segment = [System.ArraySegment[byte]]::new($buffer)
    $cts = [System.Threading.CancellationTokenSource]::new([TimeSpan]::FromSeconds($TimeoutSeconds))
    $result = $Socket.ReceiveAsync($segment, $cts.Token).GetAwaiter().GetResult()

    if ($result.MessageType -eq [System.Net.WebSockets.WebSocketMessageType]::Close) {
        throw "WebSocket closed before receiving message"
    }

    [System.Text.Encoding]::UTF8.GetString($buffer, 0, $result.Count)
}

$health = Invoke-RestMethod -Uri "$BaseUrl/health"
Write-Host "Health version: $($health.version)"

$socket = [System.Net.WebSockets.ClientWebSocket]::new()
$socket.ConnectAsync([Uri]$WsUrl, [Threading.CancellationToken]::None).GetAwaiter().GetResult() | Out-Null

$initial = Receive-WebSocketText -Socket $socket
Write-Host "Initial state: $initial"

$payload = @{
    id_recorrido = (Get-Random -Minimum 100000 -Maximum 999999)
    id_usuario = 1
    id_estacion = 10
    operacion = "retiro"
    fechahora = (Get-Date).ToUniversalTime().ToString("yyyy-MM-ddTHH:mm:ssZ")
} | ConvertTo-Json -Compress

Invoke-RestMethod `
    -Uri "$BaseUrl/rabbit/mensajes" `
    -Method Post `
    -Headers @{ "X-Usuario" = $Usuario; "X-Contrasena" = $Contrasena } `
    -ContentType "application/json" `
    -Body $payload | Out-Null

$update = Receive-WebSocketText -Socket $socket
Write-Host "Update state: $update"

$json = $update | ConvertFrom-Json
if (-not $json.movimiento) {
    throw "Expected movement in WebSocket update"
}

Write-Host "Smoke OK: published message and received WebSocket update"
