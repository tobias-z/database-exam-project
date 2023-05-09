import { NextResponse } from "next/server";

export async function GET(request: Request) {
    const { searchParams } = new URL(request.url);
    const path = searchParams.get("path");
    if (!path) {
        return respond(400, "missing 'path' query param");
    }

    const logServiceUrl = process.env.LOG_SERVICE_URL;

    if (!logServiceUrl) {
        console.log("ENV variable LOG_SERVICE_URL was undefined. Sending error response in proxy");
        return respond(500, "Internal Server Error");
    }

    try {
        const res = await fetch(
            `${logServiceUrl}${path}`,
            Object.fromEntries(request.headers.entries())
        );

        if (res.status < 400) {
            return NextResponse.json(await res.json());
        }
        return respond(res.status, await res.json())
    } catch (_) {
        return respond(400, "invalid request");
    }
}

function respond(status: number, message: string) {
    return new Response(JSON.stringify({ message }), {
        status,
        headers: {
            "content-type": "application/json",
        },
    });
}
