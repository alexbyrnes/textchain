
function message(url) {
	var inputVal =  document.getElementById("text").value
	var data = {text: inputVal}
	 
	fetch(url, {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json'
		},
		
		body: JSON.stringify(data),
	})
		.then((response) => response.json())
.then((data) => {
  console.log('Success:', data);
})
.catch((error) => {
  console.error('Error:', error);
});
  
};


  
