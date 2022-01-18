$(document).ready(function() {

    $('#input_username').keyup(function () {
        isFill("#input_username", "#error_username", "need a username");
    });

    $('#input_passwordFirst').keyup(function () {
        isFill("#input_passwordFirst", "#error_passwordFirst", "need a password");
        isSame();
    });


    $('#input_passwordSecond').keyup(function () {
        isFill("#input_passwordSecond", "#error_passwordSecond", "need to confirm password");
        isSame();
    });

    $('#input_email').keyup(function () {
       isFill("#input_email", "#error_email", "need an email");
    });
});

function isSame() {
    isMatch($('#input_passwordFirst'), $('#input_passwordSecond'), $('#error_passwordSecond'));
}

function init() {
    isFill("#input_username", "#error_username", "need a username");
    isFill("#input_passwordFirst", "#error_passwordFirst", "need a password");
    isFill("#input_passwordSecond", "#error_passwordSecond", "need to confirm password");
    isFill("#input_email", "#error_email", "need an email");
    isSame();
}