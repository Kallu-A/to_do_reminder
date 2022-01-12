function hide_after_time() {
    window.setTimeout(function() {
        $('#close_button').fadeTo(1000, 0).slideUp(1000, function(){
            $(this).remove()
        });
    }, 1500);
}

function confirm_pop_up() {
    confirm("this is a confirm");
}

function set_upload() {
    const fileName = document.querySelector(".file-name");
    const uploadBtn = document.querySelector("#upload_button");
    const preview = document.querySelector("preview");

    uploadBtn.addEventListener("change", function(){
        const file = this.files[0];
        if(file){
            const reader = new FileReader();
            reader.onload = function(){
                const result = reader.result;
                preview.src = result;
            }
            cancelBtn.addEventListener("click", function(){
                img.src = "";
            })
            reader.readAsDataURL(file);
        }
    });
}


