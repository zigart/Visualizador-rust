const disponibles = document.querySelector("#bicicletas-disponibles");
const enUso = document.querySelector("#bicicletas-en-uso");
const ultimaActualizacion = document.querySelector("#ultima-actualizacion");
const estadoConexion = document.querySelector("#estado-conexion");
const historial = document.querySelector("#historial-eventos");

function wsUrl() {
  const protocol = window.location.protocol === "https:" ? "wss:" : "ws:";
  return `${protocol}//${window.location.host}/ws`;
}

function formatDate(value) {
  return new Intl.DateTimeFormat("es-AR", {
    dateStyle: "short",
    timeStyle: "medium",
  }).format(new Date(value));
}

function setConnection(label, offline = false) {
  estadoConexion.textContent = label;
  estadoConexion.classList.toggle("offline", offline);
}

function renderEstado(estado) {
  disponibles.textContent = estado.bicicletas_disponibles;
  enUso.textContent = estado.bicicletas_en_uso;
  ultimaActualizacion.textContent = formatDate(estado.actualizado_en);
  setConnection(estado.estado_conexion || "Conectado", false);

  if (estado.movimiento) {
    const item = document.createElement("li");
    item.textContent = `${formatDate(estado.movimiento.fechahora)} - ${estado.movimiento.operacion} recorrido ${estado.movimiento.id_recorrido} usuario ${estado.movimiento.id_usuario}`;
    historial.prepend(item);
  }
}

function connect() {
  const socket = new WebSocket(wsUrl());

  socket.addEventListener("open", () => setConnection("Conectado", false));
  socket.addEventListener("message", (event) => renderEstado(JSON.parse(event.data)));
  socket.addEventListener("close", () => {
    setConnection("Desconectado", true);
    window.setTimeout(connect, 1500);
  });
  socket.addEventListener("error", () => setConnection("Error de conexion", true));
}

connect();
