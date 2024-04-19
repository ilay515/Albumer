import React from 'react'
import PageLayout from './components/page-layout/page-layout'
import { Flex } from 'antd'

const App: React.FC = () => (
  <Flex gap="middle" wrap="wrap">
    <PageLayout />
  </Flex>
);

export default App