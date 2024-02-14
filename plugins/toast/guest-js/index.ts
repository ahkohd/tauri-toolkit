import { invoke } from "@tauri-apps/api/tauri";

export enum ToastPosition {
	Top = 0,
	Bottom = 1,
}

export interface ToastOptions {
	paddingX: number;
	paddingY: number;
	duration: number;
	position: ToastPosition;
	distance: number;
	fontSize: number;
	marginBetweenToasts: number;
	enterAnimationDuration: number;
	exitAnimationDuration: number;
}

export const toast = async (message: string, options?: ToastOptions) =>
	invoke("plugin:toast|toast", {
		message,
		options,
	});
