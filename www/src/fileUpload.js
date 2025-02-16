// Prompt a user to upload a file, return the contents as a string
// if `acceptedFileExtensionsString` is provided, the user can only upload certain filetypes
// example val: ".xml" (for just one extension) or ".xml,.csv" (for multiple)
export const readFileFromUser = (acceptedFileExtensionsString = null) => {
	return new Promise((resolve) => {
		const fileInput = document.createElement("input");
		fileInput.type = "file";
		if (acceptedFileExtensionsString !== null) {
			fileInput.accept = acceptedFileExtensionsString;
		}
		fileInput.addEventListener("change", (event) => {
			const file = event.target.files[0];
			const reader = new FileReader();
			reader.onabort = () => {
				resolve(null);
			};
			reader.onload = () => {
				resolve({
					name: file.name,
					data: reader.result,
				});
			};
			reader.readAsArrayBuffer(file);
		});
		fileInput.click();
	});
};
