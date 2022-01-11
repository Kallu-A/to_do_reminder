function hide_after_time() {
    window.setTimeout(function() {
        $('#close_button').fadeTo(1000, 0).slideUp(1000, function(){
            $(this).remove()
        });
    }, 2000);
}