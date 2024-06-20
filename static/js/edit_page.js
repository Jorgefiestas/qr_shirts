document.addEventListener("DOMContentLoaded", function () {
  turnReadOnlyOff();

  document.querySelectorAll("textarea").forEach((textarea) => {
    textarea.addEventListener("keyup", textAreaAdjust);
  });

  document
    .getElementById("save-button")
    .addEventListener("click", submitChanges);
});

function textAreaAdjust(element) {
  element.style.height = "1px";
  element.style.height = 25 + element.scrollHeight + "px";
}

function turnReadOnlyOff() {
  const parameter_elements = document.getElementsByClassName("parameter");
  Array.from(parameter_elements).forEach((parameter) => {
    parameter.readOnly = false;
  });
}

async function submitChanges() {
  const secret = window.location.href.split("/").pop();
  const qr_id = document.getElementById("shirt-id").value;
  const template_id = document.getElementById("template-id").value;

  const parameter_elements = document.getElementsByClassName("parameter");
  const parameters = {};
  Array.from(parameter_elements).forEach((parameter) => {
    parameters[parameter.id] = parameter.value;
  });

  const request_body = {
    secret: secret,
    page: {
      id: qr_id,
      template_id: Number(template_id),
      parameters: parameters,
    },
  };

  const response = await fetch("/update", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(request_body),
  });

  if (response.ok) {
    alert("Update was successfull.");
  } else {
    alert("Update failed.");
  }
}
