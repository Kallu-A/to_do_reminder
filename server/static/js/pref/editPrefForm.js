$(document).ready(function() {

    $('#input_mode').keyup(function () {
        isFill("#input_mode", "#error_mode", "need a mode");
    });


    $('#input_display').keyup(function () {
        isFill("#input_display", "#error_display", "need a value");
    });
});


function init_email() {
    isEmail($('#input_email'), $('#error_email'));
}

function toogle_mode() {
    const display = $('#form_display');
    const mode = $('#form_mode');
    const title_mode = $('#title_mode');
    const title_display = $('#title_display');

    if (mode.is(":hidden")) {
        mode.show("mid");
        display.hide("mid");

        title_mode.removeClass().addClass("selected_title");
        title_display.removeClass().addClass("not_selected");
    } else {
        mode.hide("mid");
        //email.removeClass().addClass("inactive");
        title_mode.removeClass().addClass("not_selected");
    }

}

function toogle_display() {
    const display = $('#form_display');
    const mode = $('#form_mode');
    const title_mode = $('#title_mode');
    const title_display = $('#title_display');

    if (display.is(":hidden")) {
        //password.removeClass().addClass("active");
        //email.removeClass().addClass("inactive");
        display.show("mid");
        mode.hide("mid");

        title_display.removeClass().addClass("selected_title");
        title_mode.removeClass().addClass("not_selected");

    } else {
        display.hide("mid");
        //password.removeClass().addClass("inactive");
        title_display.removeClass().addClass("not_selected");
    }

}