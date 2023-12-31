import { writable } from 'svelte/store';

let screenSize = writable('lg');

if (typeof window !== 'undefined') {
	function handleResize() {
		if (window.innerWidth < 639) {
			screenSize.set('sm');
		}
		else {
			screenSize.set('lg');
		}
	}
	window.addEventListener('resize', handleResize);
}

export default screenSize;
