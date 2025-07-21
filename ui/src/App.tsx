import React from 'react';
import { BrowserRouter as Router, Routes, Route, Navigate } from 'react-router-dom';
import { MainLayout } from './components/layout';
import { Overview, DataSource } from './pages';

const App: React.FC = () => {
  return (
    <Router>
      <MainLayout>
        <Routes>
          {/* 默认重定向到概览页面 */}
          <Route path="/" element={<Navigate to="/overview" replace />} />
          
          {/* 概览页面 */}
          <Route path="/overview" element={<Overview />} />
          
          {/* 数据源管理页面 */}
          <Route path="/datasource" element={<DataSource />} />
          
          {/* 404 页面 - 重定向到概览 */}
          <Route path="*" element={<Navigate to="/overview" replace />} />
        </Routes>
      </MainLayout>
    </Router>
  );
};

export default App;
