$(document).ready(function() {
    handle_selector();
    $('#input_title').keyup(function () {
        isFill("#input_title", "#error_title", "need a title");
    });

    $('#input_progress').keyup(function () {
       isFill("#input_progress", "#error_progress", "need a digital value");
    });

    $('#input_priority').keyup(function () {
        isFill("#input_priority", "#display_priority", "need a priority");
    });

    $("#input_priority").keyup(function(){
        handle_selector();
    })

    $("#input_priority").click(function(){
        handle_selector();
    })

    $("#input_priority").mousemove(function(){
        handle_selector();
    })

});


function init() {
    isFill("#input_title", "#error_title", "need a title");
    isFill("#input_date", "#error_date", "need a date");
    isFill("#input_priority", "#error_priority", "need a priority");
    isFill("#input_progress", "#error_progress", "need a digital value");
    handle_selector();
}
