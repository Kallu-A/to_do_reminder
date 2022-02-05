// Script for the form forgot_password
$(document).ready(function() {

    $('#input_username').keyup(function () {
        isFill("#input_username", "#error_username", "need a username");
    });

    $('#input_email').keyup(function () {
        isEmail($('#input_email'), $('#error_email'));
    });
});



function init() {
    isEmail($('#input_email'), $('#error_email'));
    isFill("#input_username", "#error_username", "need a username");
}
