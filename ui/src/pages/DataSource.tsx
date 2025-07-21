import React, { useState } from 'react';
import { Button, Card, Input, Modal } from '../components/ui';
import { StatusIndicator } from '../components/features';
import { useAppStore } from '../store';

const DataSource: React.FC = () => {
  const { dataSources, addDataSource, getStatistics } = useAppStore();
  const stats = getStatistics();

  const [showAddModal, setShowAddModal] = useState(false);
  const [newSourcePath, setNewSourcePath] = useState('');

  const getStatusColor = (status: string) => {
    switch (status) {
      case 'connected': return 'success';
      case 'disconnected': return 'warning';
      case 'error': return 'danger';
      default: return 'info';
    }
  };

  const getStatusText = (status: string) => {
    switch (status) {
      case 'connected': return '已连接';
      case 'disconnected': return '未连接';
      case 'error': return '错误';
      default: return '未知';
    }
  };

  const handleAddDataSource = () => {
    if (newSourcePath.trim()) {
      addDataSource({
        name: `数据源 ${dataSources.length + 1}`,
        path: newSourcePath.trim(),
        status: 'disconnected',
        lastSync: new Date().toLocaleString('zh-CN'),
        messageCount: 0,
        contactCount: 0,
        size: 0
      });
      setNewSourcePath('');
      setShowAddModal(false);
    }
  };

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-bold text-gray-900">数据源管理</h1>
          <p className="text-gray-600 mt-1">管理微信数据源连接和同步设置</p>
        </div>
        <Button 
          onClick={() => setShowAddModal(true)}
          className="flex items-center gap-2"
        >
          <i className="fas fa-plus"></i>
          添加数据源
        </Button>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
        <Card className="p-6">
          <div className="flex items-center">
            <div className="p-3 bg-blue-100 rounded-lg">
              <i className="fas fa-database text-blue-600 text-xl"></i>
            </div>
            <div className="ml-4">
              <p className="text-sm text-gray-600">总数据源</p>
              <p className="text-2xl font-bold text-gray-900">{stats.totalDataSources}</p>
            </div>
          </div>
        </Card>

        <Card className="p-6">
          <div className="flex items-center">
            <div className="p-3 bg-green-100 rounded-lg">
              <i className="fas fa-check-circle text-green-600 text-xl"></i>
            </div>
            <div className="ml-4">
              <p className="text-sm text-gray-600">已连接</p>
              <p className="text-2xl font-bold text-gray-900">
                {stats.connectedDataSources}
              </p>
            </div>
          </div>
        </Card>

        <Card className="p-6">
          <div className="flex items-center">
            <div className="p-3 bg-orange-100 rounded-lg">
              <i className="fas fa-comments text-orange-600 text-xl"></i>
            </div>
            <div className="ml-4">
              <p className="text-sm text-gray-600">总消息数</p>
              <p className="text-2xl font-bold text-gray-900">
                {stats.totalMessages.toLocaleString()}
              </p>
            </div>
          </div>
        </Card>
      </div>

      <Card>
        <div className="p-6 border-b border-gray-200">
          <h3 className="text-lg font-semibold text-gray-900">数据源列表</h3>
        </div>
        <div className="divide-y divide-gray-200">
          {dataSources.map((source) => (
            <div key={source.id} className="p-6">
              <div className="flex items-center justify-between">
                <div className="flex-1">
                  <div className="flex items-center gap-3 mb-2">
                    <h4 className="text-lg font-medium text-gray-900">{source.name}</h4>
                    <StatusIndicator 
                      status={getStatusColor(source.status) as any}
                      text={getStatusText(source.status)}
                    />
                  </div>
                  <p className="text-sm text-gray-600 mb-3">{source.path}</p>
                  <div className="grid grid-cols-1 md:grid-cols-3 gap-4 text-sm">
                    <div>
                      <span className="text-gray-500">最后同步:</span>
                      <span className="ml-2 text-gray-900">{source.lastSync}</span>
                    </div>
                    <div>
                      <span className="text-gray-500">消息数:</span>
                      <span className="ml-2 text-gray-900">{source.messageCount.toLocaleString()}</span>
                    </div>
                    <div>
                      <span className="text-gray-500">联系人:</span>
                      <span className="ml-2 text-gray-900">{source.contactCount}</span>
                    </div>
                  </div>
                </div>
                <div className="flex items-center gap-2 ml-6">
                  <Button
                    variant="outline"
                    size="sm"
                    isDisabled={source.status === 'error'}
                  >
                    <i className="fas fa-sync mr-1"></i>
                    同步
                  </Button>
                  <Button 
                    variant="outline" 
                    size="sm"
                  >
                    <i className="fas fa-cog mr-1"></i>
                    设置
                  </Button>
                  <Button 
                    variant="outline" 
                    size="sm"
                    className="text-red-600 hover:text-red-700"
                  >
                    <i className="fas fa-trash mr-1"></i>
                    删除
                  </Button>
                </div>
              </div>
            </div>
          ))}
        </div>
      </Card>

      {/* 添加数据源模态框 */}
      <Modal
        isOpen={showAddModal}
        onClose={() => setShowAddModal(false)}
        title="添加数据源"
      >
        <div className="space-y-4">
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-2">
              微信数据路径
            </label>
            <Input
              type="text"
              value={newSourcePath}
              onChange={(e) => setNewSourcePath(e.target.value)}
              placeholder="例如: C:\Users\User\Documents\WeChat Files\wxid_xxx"
              className="w-full"
            />
            <p className="text-xs text-gray-500 mt-1">
              请选择微信数据文件夹路径
            </p>
          </div>
          <div className="flex justify-end gap-3 pt-4">
            <Button
              variant="outline"
              onClick={() => setShowAddModal(false)}
            >
              取消
            </Button>
            <Button
              onPress={handleAddDataSource}
              isDisabled={!newSourcePath.trim()}
            >
              添加
            </Button>
          </div>
        </div>
      </Modal>
    </div>
  );
};

export default DataSource;