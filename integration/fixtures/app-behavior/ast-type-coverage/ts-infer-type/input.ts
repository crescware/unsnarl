type T<X> = X extends Array<infer U> ? U : never;
