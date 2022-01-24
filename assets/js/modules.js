let storedModules = JSON.parse(localStorage.getItem("modulesViewed"));
if (storedModules) {
  for (let i = 0; i < storedModules.length; i++) {
  let tickmarkId = document.getElementById(`tickmark-${storedModules[i]}`);
    if (tickmarkId) {
      tickmarkId.classList.add("tickShow");
    }
  }
}
