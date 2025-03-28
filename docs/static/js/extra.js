//图片放大//
document.querySelectorAll(".zoom").forEach((item) => {
  item.addEventListener("click", function () {
    this.classList.toggle("image-zoom-large");
  });
});
