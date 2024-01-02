
<script lang='ts'> 
let file:File;
$: url=""
async function fileUpload(e){
	file = e.target.files[0];
	const blob = new Blob([file], { type: file.type });
	console.log(blob)
	const blobArray = await blob.arrayBuffer()
	console.log("blob",blobArray)
	const newArrayHeader = blobArray.slice(0, 25000)
	const newArrayBody = blobArray.slice(200000,300000)
	console.log("newArrayHeader",newArrayHeader)
	console.log("newArrayBody",newArrayBody)
	
	try {
		const newBlob = new  Blob([newArrayHeader,newArrayBody], { type: file.type });
		const newUrlFromBlob = URL.createObjectURL(newBlob);
		console.log(newUrlFromBlob)
		url=newUrlFromBlob

	} catch (e){
		console.log(e)
	}

	// console.log(file)
	// const urlFromFile = URL.createObjectURL(file); 
	// console.log(urlFromFile)
	
}
</script>
<input type="file" on:change={async (e)=>await fileUpload(e)}>
<video src="{url}" controls></video>