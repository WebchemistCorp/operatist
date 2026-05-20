import React from 'react'
import { Hero } from '@/features/landing/Hero'
import { About } from '@/features/landing/About'
import { Features } from '@/features/landing/Features'
import { CTA } from '@/features/landing/CTA'

const LandingPage: React.FC = () => (
  <main>
    <Hero />
    <About />
    <Features />
    <CTA />
  </main>
)

export default LandingPage
