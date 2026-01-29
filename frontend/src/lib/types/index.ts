export interface DataRow {
	[key: string]: string | number | boolean | null;
}

export interface ProcessingRequest {
	data: DataRow[];
	transformationType: TransformationType;
	parameters?: ProcessingParameters;
}

export interface ProcessingResponse {
	success: boolean;
	result: DataRow[];
	message: string;
	metadata?: {
		transformation: string;
		input_rows: number;
		output_rows: number;
	};
}

export interface ErrorResponse {
	error: string;
	message: string;
	details?: unknown;
}

export type TransformationType =
	| 'normalize'
	| 'aggregate'
	| 'filter'
	| 'transform'
	| 'sort'
	| 'deduplicate';

export interface ProcessingParameters {
	field?: string;
	groupBy?: string;
	operation?: string;
	condition?: string;
	value?: string | number;
	direction?: 'asc' | 'desc';
	fields?: string[];
}

export interface FileUploadState {
	file: File | null;
	data: DataRow[];
	isLoading: boolean;
	error: string | null;
}

export interface ProcessingState {
	isProcessing: boolean;
	result: DataRow[] | null;
	error: string | null;
	transformationType: TransformationType | null;
	parameters: ProcessingParameters | null;
}

export interface TransformationOption {
	value: TransformationType;
	label: string;
	description: string;
	parameters: ParameterDefinition[];
}

export interface ParameterDefinition {
	name: string;
	label: string;
	type: 'text' | 'select' | 'number' | 'multiselect';
	required: boolean;
	options?: { value: string; label: string }[];
	placeholder?: string;
	description?: string;
}

export const TRANSFORMATION_OPTIONS: TransformationOption[] = [
	{
		value: 'normalize',
		label: 'Normalize',
		description: 'Normalize numeric values in a specified field to the range [0, 1]',
		parameters: [
			{
				name: 'field',
				label: 'Field to normalize',
				type: 'text',
				required: true,
				placeholder: 'e.g., value, price, score',
				description: 'The field containing numeric values to normalize'
			}
		]
	},
	{
		value: 'aggregate',
		label: 'Aggregate',
		description: 'Group data by a field and aggregate values',
		parameters: [
			{
				name: 'groupBy',
				label: 'Group by field',
				type: 'text',
				required: true,
				placeholder: 'e.g., category, region',
				description: 'The field to group rows by'
			},
			{
				name: 'field',
				label: 'Field to aggregate',
				type: 'text',
				required: true,
				placeholder: 'e.g., amount, quantity',
				description: 'The field containing values to aggregate'
			},
			{
				name: 'operation',
				label: 'Aggregation operation',
				type: 'select',
				required: true,
				options: [
					{ value: 'sum', label: 'Sum' },
					{ value: 'avg', label: 'Average' },
					{ value: 'min', label: 'Minimum' },
					{ value: 'max', label: 'Maximum' },
					{ value: 'count', label: 'Count' },
					{ value: 'median', label: 'Median' }
				],
				description: 'How to combine values within each group'
			}
		]
	},
	{
		value: 'filter',
		label: 'Filter',
		description: 'Filter rows based on a condition',
		parameters: [
			{
				name: 'field',
				label: 'Field to filter on',
				type: 'text',
				required: true,
				placeholder: 'e.g., status, category',
				description: 'The field to evaluate the condition against'
			},
			{
				name: 'condition',
				label: 'Condition',
				type: 'select',
				required: true,
				options: [
					{ value: 'equals', label: 'Equals' },
					{ value: 'notequals', label: 'Not Equals' },
					{ value: 'greaterthan', label: 'Greater Than' },
					{ value: 'lessthan', label: 'Less Than' },
					{ value: 'greaterorequal', label: 'Greater or Equal' },
					{ value: 'lessorequal', label: 'Less or Equal' },
					{ value: 'contains', label: 'Contains' },
					{ value: 'startswith', label: 'Starts With' },
					{ value: 'endswith', label: 'Ends With' }
				],
				description: 'The comparison operation to apply'
			},
			{
				name: 'value',
				label: 'Comparison value',
				type: 'text',
				required: true,
				placeholder: 'e.g., active, 100',
				description: 'The value to compare against'
			}
		]
	},
	{
		value: 'transform',
		label: 'Transform',
		description: 'Apply mathematical transformations to numeric fields',
		parameters: [
			{
				name: 'field',
				label: 'Field to transform',
				type: 'text',
				required: true,
				placeholder: 'e.g., value, amount',
				description: 'The field containing numeric values to transform'
			},
			{
				name: 'operation',
				label: 'Mathematical operation',
				type: 'select',
				required: true,
				options: [
					{ value: 'square', label: 'Square (x²)' },
					{ value: 'sqrt', label: 'Square Root (√x)' },
					{ value: 'log', label: 'Natural Logarithm (ln x)' },
					{ value: 'exp', label: 'Exponential (eˣ)' },
					{ value: 'abs', label: 'Absolute Value (|x|)' },
					{ value: 'negate', label: 'Negate (-x)' },
					{ value: 'reciprocal', label: 'Reciprocal (1/x)' }
				],
				description: 'The mathematical operation to apply'
			}
		]
	},
	{
		value: 'sort',
		label: 'Sort',
		description: 'Sort data by a specified field',
		parameters: [
			{
				name: 'field',
				label: 'Field to sort by',
				type: 'text',
				required: true,
				placeholder: 'e.g., date, name, value',
				description: 'The field to use for sorting'
			},
			{
				name: 'direction',
				label: 'Sort direction',
				type: 'select',
				required: true,
				options: [
					{ value: 'asc', label: 'Ascending' },
					{ value: 'desc', label: 'Descending' }
				],
				description: 'The order in which to sort'
			}
		]
	},
	{
		value: 'deduplicate',
		label: 'Deduplicate',
		description: 'Remove duplicate rows based on specified fields',
		parameters: [
			{
				name: 'fields',
				label: 'Fields to consider (optional)',
				type: 'text',
				required: false,
				placeholder: 'e.g., id, email (comma-separated)',
				description: 'Leave empty to compare all fields, or specify which fields to use for duplicate detection'
			}
		]
	}
];
