$(document).ready(function() {

    $('#input_title').keyup(function () {
        isFill("#input_title", "#error_title", "need a title");
    });


    $('#input_priority').keyup(function () {
        isFill("#input_priority", "#error_priority", "need a priority");
    });

});

function init() {
    isFill("#input_title", "#error_title", "need a title");
    isFill("#input_date", "#error_date", "need a date");
    isFill("#input_priority", "#error_priority", "need a priority");
}