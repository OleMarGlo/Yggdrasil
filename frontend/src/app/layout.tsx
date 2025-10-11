import type { Metadata } from 'next'
import './globals.css'
import NavBar from '@/components/navbar/nav'
import Footer from '@/components/footer/footer'

export const metadata: Metadata = {
    title: 'Yggdrasil',
    description: 'Yggdrasil - Portfilio website',
}

export default function RootLayout({
    children,
}: Readonly<{
    children: React.ReactNode;
}>) {
    return (
        <html lang="en">
            <body className="flex h-screen flex-col bg-yggdrasil-950">
                <div className="h-16">
                    <NavBar />
                </div>

                <div className="flex flex-col flex-1 overflow-y-auto max-w-7xl mx-auto w-full bg-yggdrasil-900">
                    <main className="flex-grow w-full">
                        {children}
                    </main>
                    <Footer />
                </div>
            </body>
        </html>
    );
}
