import { useQuery } from "react-query";

type ProxyError = {
    message: {
        code: number;
        message: string;
    };
};

export default function useLogServiceProxy<T>(
    key: string,
    path: string,
    transformationFn?: (data: T) => T
) {
    return useQuery<T, ProxyError>(
        `proxy-${key}`,
        async () => {
            return fetch(`/api/proxy?path=${path}`).then(async res => {
                if (res.status > 400) {
                    return Promise.reject(await res.json());
                }
                return res.json();
            });
        },
        { enabled: false, select: transformationFn, retry: false }
    );
}
