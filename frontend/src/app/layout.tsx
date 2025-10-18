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
        <html lang='en'>
            <body className='flex h-screen flex-col bg-yggdrasil-950'>
                <NavBar />
                <div className='flex flex-col flex-1 overflow-y-scroll no-scrollbar max-w-7xl mx-auto bg-yggdrasil-900'>
                    <main className='flex-grow w-full py-16 '>
                        {children}
                    </main>
                    <Footer />
                </div>
            </body>
        </html>
    )
}
