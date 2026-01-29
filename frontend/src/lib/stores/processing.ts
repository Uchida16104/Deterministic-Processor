import { writable, derived } from 'svelte/store';
import type {
	DataRow,
	ProcessingState,
	FileUploadState,
	TransformationType,
	ProcessingParameters,
	ProcessingRequest,
	ProcessingResponse
} from '../types';

export const fileUploadState = writable<FileUploadState>({
	file: null,
	data: [],
	isLoading: false,
	error: null
});

export const processingState = writable<ProcessingState>({
	isProcessing: false,
	result: null,
	error: null,
	transformationType: null,
	parameters: null
});

export const hasData = derived(fileUploadState, ($state) => $state.data.length > 0);

export const hasResult = derived(processingState, ($state) => $state.result !== null);

export const canProcess = derived(
	[fileUploadState, processingState],
	([$fileState, $procState]) =>
		$fileState.data.length > 0 && !$procState.isProcessing && $procState.transformationType !== null
);

export function setUploadedData(data: DataRow[]): void {
	fileUploadState.update((state) => ({
		...state,
		data,
		error: null
	}));
}

export function setUploadError(error: string): void {
	fileUploadState.update((state) => ({
		...state,
		error,
		isLoading: false
	}));
}

export function setUploadLoading(isLoading: boolean): void {
	fileUploadState.update((state) => ({
		...state,
		isLoading
	}));
}

export function setTransformationType(type: TransformationType): void {
	processingState.update((state) => ({
		...state,
		transformationType: type,
		parameters: null,
		result: null,
		error: null
	}));
}

export function setParameters(parameters: ProcessingParameters): void {
	processingState.update((state) => ({
		...state,
		parameters
	}));
}

export function setProcessingResult(result: DataRow[]): void {
	processingState.update((state) => ({
		...state,
		result,
		isProcessing: false,
		error: null
	}));
}

export function setProcessingError(error: string): void {
	processingState.update((state) => ({
		...state,
		error,
		isProcessing: false
	}));
}

export function setProcessing(isProcessing: boolean): void {
	processingState.update((state) => ({
		...state,
		isProcessing
	}));
}

export function resetAll(): void {
	fileUploadState.set({
		file: null,
		data: [],
		isLoading: false,
		error: null
	});
	
	processingState.set({
		isProcessing: false,
		result: null,
		error: null,
		transformationType: null,
		parameters: null
	});
}

export function resetResults(): void {
	processingState.update((state) => ({
		...state,
		result: null,
		error: null
	}));
}
