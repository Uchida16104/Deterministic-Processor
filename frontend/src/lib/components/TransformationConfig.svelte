<script lang="ts">
	import { onMount } from 'svelte';
	import {
		processingState,
		setTransformationType,
		setParameters,
		setProcessing,
		setProcessingResult,
		setProcessingError,
		fileUploadState
	} from '../stores/processing';
	import { TRANSFORMATION_OPTIONS } from '../types';
	import type { TransformationType, ProcessingParameters } from '../types';
	import { PUBLIC_API_URL } from '$env/static/public';

	let selectedTransformation: TransformationType | null = null;
	let parameters: Record<string, any> = {};
	let Alpine: any;

	onMount(async () => {
		const alpineModule = await import('alpinejs');
		Alpine = alpineModule.default;
	});

	$: selectedOption = TRANSFORMATION_OPTIONS.find((opt) => opt.value === selectedTransformation);

	function handleTransformationChange(event: Event) {
		const target = event.target as HTMLSelectElement;
		const value = target.value as TransformationType;
		selectedTransformation = value;
		setTransformationType(value);
		parameters = {};
	}

	function handleParameterChange(paramName: string, value: any) {
		parameters = { ...parameters, [paramName]: value };
	}

	async function handleSubmit() {
		if (!selectedTransformation || !$fileUploadState.data.length) {
			return;
		}

		const processedParams: ProcessingParameters = {};
		
		if (selectedOption) {
			for (const param of selectedOption.parameters) {
				if (param.required && !parameters[param.name]) {
					setProcessingError(`Required parameter "${param.label}" is missing`);
					return;
				}
				
				if (parameters[param.name] !== undefined) {
					if (param.name === 'fields' && typeof parameters[param.name] === 'string') {
						processedParams[param.name] = parameters[param.name]
							.split(',')
							.map((f: string) => f.trim())
							.filter((f: string) => f.length > 0);
					} else if (param.type === 'number') {
						processedParams[param.name] = Number(parameters[param.name]);
					} else {
						processedParams[param.name] = parameters[param.name];
					}
				}
			}
		}

		setProcessing(true);
		setParameters(processedParams);

		try {
			const response = await fetch(`${PUBLIC_API_URL}/api/process`, {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json'
				},
				body: JSON.stringify({
					data: $fileUploadState.data,
					transformationType: selectedTransformation,
					parameters: processedParams
				})
			});

			if (!response.ok) {
				const errorData = await response.json();
				throw new Error(errorData.message || 'Processing failed');
			}

			const result = await response.json();
			setProcessingResult(result.result);
		} catch (error) {
			const errorMessage = error instanceof Error ? error.message : 'An unknown error occurred';
			setProcessingError(errorMessage);
		}
	}
</script>

<div class="bg-white rounded-lg shadow-sm border border-gray-200 p-6">
	<h2 class="text-lg font-semibold text-gray-900 mb-4">Configure Transformation</h2>

	<form on:submit|preventDefault={handleSubmit} class="space-y-6">
		<div>
			<label for="transformation" class="block text-sm font-medium text-gray-700 mb-2">
				Transformation Type
			</label>
			<select
				id="transformation"
				bind:value={selectedTransformation}
				on:change={handleTransformationChange}
				class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-primary-500 transition-colors duration-200"
				disabled={$fileUploadState.data.length === 0}
			>
				<option value={null}>Select a transformation...</option>
				{#each TRANSFORMATION_OPTIONS as option}
					<option value={option.value}>{option.label}</option>
				{/each}
			</select>
			{#if selectedOption}
				<p class="mt-2 text-sm text-gray-600">{selectedOption.description}</p>
			{/if}
		</div>

		{#if selectedOption}
			<div class="space-y-4 animate-fade-in">
				<h3 class="text-sm font-semibold text-gray-900">Parameters</h3>

				{#each selectedOption.parameters as param}
					<div>
						<label for={param.name} class="block text-sm font-medium text-gray-700 mb-2">
							{param.label}
							{#if param.required}
								<span class="text-red-500">*</span>
							{/if}
						</label>

						{#if param.type === 'select'}
							<select
								id={param.name}
								value={parameters[param.name] || ''}
								on:change={(e) => handleParameterChange(param.name, e.currentTarget.value)}
								class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-primary-500 transition-colors duration-200"
								required={param.required}
							>
								<option value="">Select {param.label.toLowerCase()}...</option>
								{#if param.options}
									{#each param.options as option}
										<option value={option.value}>{option.label}</option>
									{/each}
								{/if}
							</select>
						{:else if param.type === 'number'}
							<input
								type="number"
								id={param.name}
								value={parameters[param.name] || ''}
								on:input={(e) => handleParameterChange(param.name, e.currentTarget.value)}
								placeholder={param.placeholder || ''}
								class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-primary-500 transition-colors duration-200"
								required={param.required}
							/>
						{:else}
							<input
								type="text"
								id={param.name}
								value={parameters[param.name] || ''}
								on:input={(e) => handleParameterChange(param.name, e.currentTarget.value)}
								placeholder={param.placeholder || ''}
								class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-primary-500 transition-colors duration-200"
								required={param.required}
							/>
						{/if}

						{#if param.description}
							<p class="mt-1 text-xs text-gray-500">{param.description}</p>
						{/if}
					</div>
				{/each}
			</div>
		{/if}

		<button
			type="submit"
			disabled={!selectedTransformation || $fileUploadState.data.length === 0 || $processingState.isProcessing}
			class="w-full px-6 py-3 bg-primary-600 text-white font-medium rounded-lg hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-primary-500 focus:ring-offset-2 disabled:opacity-50 disabled:cursor-not-allowed transition-all duration-200 flex items-center justify-center space-x-2"
		>
			{#if $processingState.isProcessing}
				<div class="animate-spin rounded-full h-5 w-5 border-b-2 border-white"></div>
				<span>Processing...</span>
			{:else}
				<svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
					<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M14 5l7 7m0 0l-7 7m7-7H3" />
				</svg>
				<span>Process Data</span>
			{/if}
		</button>

		{#if $processingState.error}
			<div class="p-4 bg-red-50 border border-red-200 rounded-lg">
				<div class="flex items-start space-x-3">
					<svg class="w-5 h-5 text-red-500 mt-0.5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
					</svg>
					<div class="flex-1">
						<h3 class="text-sm font-medium text-red-800">Processing Error</h3>
						<p class="text-sm text-red-700 mt-1">{$processingState.error}</p>
					</div>
				</div>
			</div>
		{/if}
	</form>
</div>
