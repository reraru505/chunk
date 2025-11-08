document.addEventListener('DOMContentLoaded', () => {
	const form = document.getElementById('uploadform');
	const submitButton = document.getElementById('submit_button');
	const statusDiv = document.getElementById('status');

	form.addEventListener('submit', async (event) => {
		// 1. Prevent the default form submission (which causes a page reload)
		event.preventDefault();

		// 2. Get the file from the input
		const fileInput = document.getElementById('fileUpload');
		const file = fileInput.files[0];

		if (!file) {
			statusDiv.textContent = 'Please select a file to upload.';
			statusDiv.className = 'error';
			return;
		}

		// 3. Get the custom file name from the text input
		const customFileName = document.getElementById('fileName').value;

		// 4. Create a FormData object to send the data
		const formData = new FormData();

		// The 'name' here MUST match the `name` attribute in the HTML
		// and the field name in the Rust struct.
		formData.append('file_name', customFileName);
		formData.append('file', file);

		// 5. Update UI for feedback
		statusDiv.textContent = `Uploading "${file.name}"... Please wait.`;
		statusDiv.className = '';
		submitButton.disabled = true;
		submitButton.textContent = 'Uploading...';

		try {
			// 6. Send the request using fetch to the /upload endpoint
			const response = await fetch('/upload', {
				method: 'POST',
				body: formData,
				// Do NOT set the 'Content-Type' header manually.
				// The browser will set it to 'multipart/form-data' with the correct boundary.
			});

			if (!response.ok) {
				// If server response is not 2xx, throw an error
				throw new Error(`Server responded with ${response.status}: ${response.statusText}`);
			}

			const result = await response.json(); // Your Rust handler sends back JSON
			statusDiv.textContent = `✅ Success! ${result}`;
			statusDiv.className = 'success';
			form.reset(); // Clear the form after successful upload

		} catch (error) {
			console.error('Upload failed:', error);
			statusDiv.textContent = `❌ Upload failed: ${error.message}`;
			statusDiv.className = 'error';
		} finally {
			// 7. Re-enable the button and reset its text
			submitButton.disabled = false;
			submitButton.textContent = 'Upload File';
		}
	});
});
