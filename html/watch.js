document.addEventListener('DOMContentLoaded', () => {
	const videoList = document.getElementById('video-list');
	const statusDiv = document.getElementById('status');
	const videoPlayer = document.getElementById('video-player');
	const playerContainer = document.getElementById('video-player-container');

	async function loadVideos() {
		try {
			const response = await fetch('/list-files');
			if (!response.ok) {
				throw new Error('Failed to fetch video list.');
			}
			const files = await response.json();

			statusDiv.style.display = 'none'; // Hide "Loading..."

			if (files.length === 0) {
				videoList.innerHTML = '<li>No videos found. <a href="/upload.html">Upload one now!</a></li>';
				return;
			}

			files.forEach(file => {
				const listItem = document.createElement('li');

				const link = document.createElement('a');
				link.textContent = file.name;
				link.href = '#'; // Prevent default navigation
				link.onclick = () => playVideo(file.url, file.name);

				listItem.appendChild(link);
				videoList.appendChild(listItem);
			});

		} catch (error) {
			console.error('Error loading video list:', error);
			statusDiv.textContent = 'Error loading videos.';
			statusDiv.style.color = 'red';
		}
	}

	function playVideo(url, name) {
		videoPlayer.src = url;
		videoPlayer.style.display = 'block';
		playerContainer.scrollIntoView({ behavior: 'smooth' });
		videoPlayer.play();
		console.log(`Now playing: ${name} from ${url}`);
	}

	loadVideos();
});
