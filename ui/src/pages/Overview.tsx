import React from 'react';
import { StatCard, WelcomeCard } from '../components/features';
import { Button } from '../components/ui';
import { useAppStore } from '../store';
import { formatFileSize } from '../utils';

const Overview: React.FC = () => {
  const { dataSources, getStatistics, isOnline } = useAppStore();
  const stats = getStatistics();
  return (
    <div className="space-y-6">
      {/* 页面标题 */}
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-bold text-gray-900">概览</h1>
          <p className="text-gray-600 mt-1">微信数据采集工具总览</p>
        </div>
      </div>

      {/* 欢迎卡片 */}
      <WelcomeCard 
        title="欢迎使用微信数据采集工具"
        description="高效、安全的微信数据提取和管理解决方案"
        actionText="开始使用"
        onAction={() => console.log('开始使用')}
      />

      {/* 统计卡片网格 */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
        <StatCard
          title="数据源"
          value={stats.totalDataSources.toString()}
          change={{ value: 0, type: "increase", period: "本月" }}
          icon="fas fa-database"
          color="blue"
        />
        <StatCard
          title="已提取消息"
          value={stats.totalMessages.toLocaleString()}
          change={{ value: 12.5, type: "increase", period: "本周" }}
          icon="fas fa-comments"
          color="green"
        />
        <StatCard
          title="活跃联系人"
          value={stats.totalContacts.toString()}
          change={{ value: 2.3, type: "decrease", period: "本周" }}
          icon="fas fa-users"
          color="purple"
        />
        <StatCard
          title="存储空间"
          value={formatFileSize(stats.totalSize)}
          change={{ value: 8.7, type: "increase", period: "本月" }}
          icon="fas fa-hard-drive"
          color="orange"
        />
      </div>

      {/* 快速操作区域 */}
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        <div className="bg-white rounded-lg shadow-sm border p-6">
          <h3 className="text-lg font-semibold text-gray-900 mb-4">
            <i className="fas fa-bolt text-blue-500 mr-2"></i>
            快速操作
          </h3>
          <div className="space-y-3">
            <Button
              variant="ghost"
              className="w-full justify-start p-3 rounded-lg border border-gray-200 hover:bg-gray-50 transition-colors"
              onPress={() => console.log('添加新数据源')}
            >
              <i className="fas fa-plus text-green-500 mr-3"></i>
              添加新数据源
            </Button>
            <Button
              variant="ghost"
              className="w-full justify-start p-3 rounded-lg border border-gray-200 hover:bg-gray-50 transition-colors"
              onPress={() => console.log('同步数据')}
            >
              <i className="fas fa-sync text-blue-500 mr-3"></i>
              同步数据
            </Button>
            <Button
              variant="ghost"
              className="w-full justify-start p-3 rounded-lg border border-gray-200 hover:bg-gray-50 transition-colors"
              onPress={() => console.log('导出数据')}
            >
              <i className="fas fa-download text-purple-500 mr-3"></i>
              导出数据
            </Button>
          </div>
        </div>

        <div className="bg-white rounded-lg shadow-sm border p-6">
          <h3 className="text-lg font-semibold text-gray-900 mb-4">
            <i className="fas fa-chart-line text-green-500 mr-2"></i>
            系统状态
          </h3>
          <div className="space-y-4">
            <div className="flex items-center justify-between">
              <span className="text-gray-600">数据库连接</span>
              <span className="flex items-center text-green-600">
                <i className="fas fa-circle text-xs mr-2"></i>
                正常
              </span>
            </div>
            <div className="flex items-center justify-between">
              <span className="text-gray-600">后台服务</span>
              <span className="flex items-center text-green-600">
                <i className="fas fa-circle text-xs mr-2"></i>
                运行中
              </span>
            </div>
            <div className="flex items-center justify-between">
              <span className="text-gray-600">最后同步</span>
              <span className="text-gray-500">2分钟前</span>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};

export default Overview;