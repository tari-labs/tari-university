function modulesCompleted() {
  let storedModules = JSON.parse(localStorage.getItem("modulesViewed"));
  for (let i = 0; i < storedModules.length; i++) {
    document.getElementById(`tickmark-${storedModules[i]}`).classList.add("tickShow");
  }
}
modulesCompleted();
