// Script for the form login
$(document).ready(function() {

    $('#input_username').keyup(function () {
        isFill("#input_username", "#error_username", "need a username");
    });

    $('#input_password').keyup(function () {
        isFill("#input_password", "#error_password", "need a password")
    });
});



function init() {
    isFill("#input_username", "#error_username", "need a username");
    isFill("#input_password", "#error_password", "need a password")
}