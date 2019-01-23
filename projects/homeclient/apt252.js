function fetchData(){
    console.log('made it here!');
    var url = "http://localhost:7878"
    return fetch(url, {
        method: 'GET',
        headers: {
            'Content-Type': 'text/html'
        },
        // mode: 'no-cors'
    }).then(function(res) {
        // console.log('data', res);
        // console.log('datajson ==>', res.json())
        
    //    return res.json().then(function(dataJson){
			// console.log("​fetchData -> dataJson", dataJson)
            
        // })
        return res
    })
    // .then(function(response) {
    //     console.log("​prxWebApi.httpGet -> response", response)
        
    //     return response
    // });

}


fetchData().then(function(data){
    console.log('data' , data );
})

