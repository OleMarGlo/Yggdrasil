'use client'

import SpotlightLink from './SpitlightLink'

export default function SpotlightGroup() {
    return (
        <div className='flex justify-center items-center gap-4 h-full'>
            <SpotlightLink href='/'>Posts</SpotlightLink>
            <SpotlightLink href='/'>Skills</SpotlightLink>
        </div>
    )
}
