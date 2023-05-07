import { NextResponse } from "next/server";

export async function GET(request: Request) {
    const { searchParams } = new URL(request.url);
    const path = searchParams.get("path");
    if (!path) {
        return new Response(JSON.stringify({ message: "missing 'path' query param" }), {
            status: 400,
            headers: {
                "content-type": "application/json",
            },
        });
    }

    try {
        const res = await fetch(
            `${process.env.LOG_SERVICE_URL}${path}`,
            Object.fromEntries(request.headers.entries())
        );

        return NextResponse.json(await res.json());
    } catch (_) {
        return new Response(JSON.stringify({ message: "invalid request" }), {
            status: 400,
            headers: {
                "content-type": "application/json",
            },
        });
    }
}
