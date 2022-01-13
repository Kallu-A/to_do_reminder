$(document).ready(function() {

    $('#input_username').keyup(function () {
        isFill("#input_username", "#error_username", "need a username");
    });

    $('#input_passwordFirst').keyup(function () {
        isFill("#input_passwordFirst", "#error_passwordFirst", "need a password");
        isMatch()
    });


    $('#input_passwordSecond').keyup(function () {
        isFill("#input_passwordSecond", "#error_passwordSecond", "need to confirm password");
        isMatch();
    });
});

function isMatch() {
    let passFirst = $('#input_passwordFirst');
    let passSecond = $('#input_passwordSecond');
    if ( passFirst.val() !==  passSecond.val() ) {
        passSecond.parent().removeClass().addClass("form_err");
        $('#error_passwordSecond').text("doesn't match the password");
    } else {
        if ( passFirst.val() !== "" && passSecond.val() !== "") {
            passSecond.parent().removeClass().addClass("form_ok");
            $('#error_passwordSecond').text("");
        }
    }
}

function init() {
    isFill("#input_username", "#error_username", "need a username");
    isFill("#input_passwordFirst", "#error_passwordFirst", "need a password");
    isFill("#input_passwordSecond", "#error_passwordSecond", "need to confirm password");
    isMatch();
}