import Providers from "@/utils/providers";

export const metadata = {
    title: "Logs",
    description: "Logging page to view all logs of the system",
};

export default function RootLayout({ children }: { children: React.ReactNode }) {
    return (
        <html lang="en">
            <body style={{backgroundColor: "#d2d3d4"}}>
                <Providers>{children}</Providers>
            </body>
        </html>
    );
}
