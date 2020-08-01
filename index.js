import("./pkg").then(module => {
    module.init();
    
    window.cheater = new module.WordscapesLookupWrapper();

    $("#given_letters").on("keyup", function() {
        $("#results").html(window.cheater.lookup($(this).val()));
    });
});
