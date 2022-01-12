// Script for the form login
$(document).ready(function() {

    let input_username = $('#input_username');
    input_username.keyup(function () {
        verif_username();
    });

    let input_password = $('#input_password')
    input_password.keyup(function () {
        verif_password();
    });
});

// username field
function verif_username() {
    let input_username = $('#input_username');
    let error_username = $('#error_username');
    let input = input_username.val();
    switch (input) {
        case "" :
            input_username.parent().removeClass().addClass("form_err");
            error_username.text("need a username");
            break;

        default:
            input_username.parent().removeClass().addClass("form_ok");
            error_username.text("");
    }
}

// password field
function verif_password() {
    let input_password = $('#input_password');
    let error_password = $('#error_password');
    let input = input_password.val();
    switch (input) {
        case "" :
            input_password.parent().removeClass().addClass("form_err");
            error_password.text("need a password");
            break;

        default:
            input_password.parent().removeClass().addClass("form_ok");
            error_password.text("");
    }
}

// use to set an form error with a message from the server
function set_err(name, name_text, message) {
    $(name).parent().removeClass().addClass("form_err");
    $(name_text).text(message)
}

function init() {
    verif_password();
    verif_username();
}