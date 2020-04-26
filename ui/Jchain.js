function message() {
	fetch('http://localhost:8000/add')
		.then(function(response){
						
			return response.text();
		})
.then(function(text){

});
  
};


