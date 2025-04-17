export type REnum<T> = {
    [K in keyof T]: T[K] extends object
        ? (T[K] & { _type: K })  // If object, keep as is and add `_type`
        : { _type: K; value: T[K] }; // If primitive, wrap in `{ _type, value }`
}[keyof T];  // Extract as union

export type ExclusiveEnum<T> = {
    [K in keyof T]: { [P in keyof T]: P extends K ? T[K] : never }
}[keyof T];
