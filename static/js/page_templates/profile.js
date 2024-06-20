const PAD = 30;
const WIDTH = 600;
const HEIGHT = 300;

function updateZoom() {
  var vw = window.innerWidth;
  var vh = window.innerHeight;

  if (vw > WIDTH + PAD && vh > HEIGHT + PAD) {
    document.getElementById("card").style.zoom = 1;
    return;
  }

  var fw = (vw - PAD) / WIDTH;
  var fh = (vh - PAD) / HEIGHT;
  document.getElementById("card").style.zoom = Math.min(fw, fh);
}

window.addEventListener("resize", updateZoom);
updateZoom();
