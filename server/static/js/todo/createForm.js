$(document).ready(function() {
    handle_selector();
    $('#input_title').keyup(function () {
        isFill("#input_title", "#error_title", "need a title");
    });


    $('#input_priority').keyup(function () {
        isFill("#input_priority", "#display_priority", "need a priority");
    });

    $("#input_priority").mousemove(function(){
        handle_selector();
    })
});


/// Handle the input range
function handle_selector() {
    const selector = document.getElementById("input_priority");
    const disp_selector = $('#display_priority');
    switch (selector.value) {
        case "0":
            disp_selector.text('minimum');
            disp_selector.removeClass().addClass("min");
            break;
        case "1" || "2" || "3":
            disp_selector.text("low");
            disp_selector.removeClass().addClass("low");
            break;
        case "4" || "5" || "6":
            disp_selector.text("medium");
            disp_selector.removeClass().addClass("medium");
            break;
        case "7" || "8" || "9":
            disp_selector.text("high");
            disp_selector.removeClass().addClass("high");
            break;
        case "10":
            disp_selector.text("maximum");
            disp_selector.removeClass().addClass("max");
            break;
    }
}

function init() {
    isFill("#input_title", "#error_title", "need a title");
    isFill("#input_date", "#error_date", "need a date");
    isFill("#input_priority", "#error_priority", "need a priority");
    handle_selector();
}