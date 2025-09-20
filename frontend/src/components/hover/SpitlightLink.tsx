'use client'

import Link from 'next/link'
import { useState } from 'react'

type SpotlightLinkProps = {
    href: string
    position?: 'first' | 'last'
    children: React.ReactNode
}

export default function SpotlightLink({ href, children }: SpotlightLinkProps) {
    const [hovering, setHovering] = useState(false)
    const [bgPosition, setBgPosition] = useState({ x: 0, y: 0 })

    const handleMouseMove = (e: React.MouseEvent<HTMLAnchorElement, MouseEvent>) => {
        const rect = e.currentTarget.getBoundingClientRect()
        setBgPosition({
            x: e.clientX - rect.left,
            y: e.clientY - rect.top,
        })
    }

    return (
        <Link
            href={href}
            className='relative rounded-xl px-6 py-4 cursor-pointer'
            onMouseEnter={() => setHovering(true)}
            onMouseLeave={() => setHovering(false)}
            onMouseMove={handleMouseMove}
            style={{
                background: hovering
                    ? `radial-gradient(circle 60px at ${bgPosition.x}px ${bgPosition.y}px, rgba(255,255,255,0.3), transparent 80%)`
                    : 'transparent',
                transition: 'background 0.2s',
            }}
        >
            {children}
        </Link>
    )
}
