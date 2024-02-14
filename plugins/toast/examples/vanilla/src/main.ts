import { toast } from "../../../guest-js";

let toastInputEl: HTMLInputElement | null;

async function showToast() {
	if (toastInputEl) {
		toast(toastInputEl.value);
	}
}

window.addEventListener("DOMContentLoaded", () => {
	toastInputEl = document.querySelector("#toast-input");
	document.querySelector("#toast-form")?.addEventListener("submit", (e) => {
		e.preventDefault();
		showToast();
	});
});
