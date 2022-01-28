// Hide the display flash after 1500 ms
function hide_after_time() {
    window.setTimeout(function() {
        $('#close_button').fadeTo(1000, 0).slideUp(1000, function(){
            $(this).remove()
        });
    }, 1500);
}

// Send a confirm pop_up for important action
function confirm_pop_up(message, action) {
    let choice = confirm(message);
    if (choice) {
        document.getElementById(action).click()
    }
}

// Allow to preview the new picture before saving it
function preview_img() {
    const uploadBtn = document.querySelector("#file");
    const preview = document.querySelector("#preview");

    uploadBtn.addEventListener("change", function(){
        const file = this.files[0];
        if(file){
            const reader = new FileReader();
            reader.onload = function(){
                preview.src = reader.result;
            }
            let saveBtn = document.getElementById("save_change_btn");
            saveBtn.style.visibility = "visible";
            saveBtn.style.position = "relative";
            reader.readAsDataURL(file);
        }
    });
}
