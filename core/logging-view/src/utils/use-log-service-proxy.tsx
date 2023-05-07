import { useQuery } from "react-query";

export default function useLogServiceProxy<T>(
    key: string,
    path: string,
    transformationFn?: (data: T) => T
) {
    return useQuery<T>(
        `proxy-${key}`,
        async () => {
            return fetch(`/api/proxy?path=${path}`).then(res => res.json());
        },
        { enabled: false, select: transformationFn }
    );
}
