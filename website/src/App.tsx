import React from 'react'
import { Routes, Route } from 'react-router-dom'
import PublicLayout from '@/layouts/PublicLayout'
import LandingPage from '@/pages/LandingPage'

const App: React.FC = () => (
  <Routes>
    <Route element={<PublicLayout />}>
      <Route path="/" element={<LandingPage />} />
    </Route>
  </Routes>
)

export default App
