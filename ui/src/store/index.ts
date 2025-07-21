import { create } from 'zustand';
import { devtools, persist } from 'zustand/middleware';

// 系统状态接口
interface SystemState {
  isOnline: boolean;
  lastSync: string | null;
  notifications: Notification[];
  theme: 'light' | 'dark';
}

// 数据源状态接口
interface DataSourceState {
  dataSources: DataSource[];
  selectedDataSource: string | null;
  syncStatus: 'idle' | 'syncing' | 'error';
  syncProgress: number;
}

// 通知接口
interface Notification {
  id: string;
  type: 'info' | 'success' | 'warning' | 'error';
  title: string;
  message: string;
  timestamp: string;
  read: boolean;
}

// 数据源接口
interface DataSource {
  id: string;
  name: string;
  path: string;
  status: 'connected' | 'disconnected' | 'error';
  lastSync: string;
  messageCount: number;
  contactCount: number;
  size: number;
}

// 统计数据接口
interface Statistics {
  totalDataSources: number;
  connectedDataSources: number;
  totalMessages: number;
  totalContacts: number;
  totalSize: number;
}

// 应用状态接口
interface AppState extends SystemState, DataSourceState {
  // 系统操作
  setOnlineStatus: (isOnline: boolean) => void;
  updateLastSync: (timestamp: string) => void;
  addNotification: (notification: Omit<Notification, 'id' | 'timestamp'>) => void;
  markNotificationAsRead: (id: string) => void;
  clearNotifications: () => void;
  setTheme: (theme: 'light' | 'dark') => void;

  // 数据源操作
  addDataSource: (dataSource: Omit<DataSource, 'id'>) => void;
  updateDataSource: (id: string, updates: Partial<DataSource>) => void;
  removeDataSource: (id: string) => void;
  setSelectedDataSource: (id: string | null) => void;
  setSyncStatus: (status: 'idle' | 'syncing' | 'error') => void;
  setSyncProgress: (progress: number) => void;

  // 计算属性
  getStatistics: () => Statistics;
  getUnreadNotifications: () => Notification[];
}

// 创建store
export const useAppStore = create<AppState>()(
  devtools(
    persist(
      (set, get) => ({
        // 初始状态
        isOnline: true,
        lastSync: null,
        notifications: [],
        theme: 'light',
        dataSources: [
          {
            id: '1',
            name: '主微信账号',
            path: 'C:\\Users\\User\\Documents\\WeChat Files\\wxid_123',
            status: 'connected',
            lastSync: '2024-01-20 14:30:00',
            messageCount: 1234,
            contactCount: 89,
            size: 2.4 * 1024 * 1024 * 1024 // 2.4GB
          },
          {
            id: '2',
            name: '工作微信',
            path: 'C:\\Users\\User\\Documents\\WeChat Files\\wxid_456',
            status: 'disconnected',
            lastSync: '2024-01-19 09:15:00',
            messageCount: 567,
            contactCount: 45,
            size: 1.2 * 1024 * 1024 * 1024 // 1.2GB
          },
          {
            id: '3',
            name: '备用账号',
            path: 'C:\\Users\\User\\Documents\\WeChat Files\\wxid_789',
            status: 'error',
            lastSync: '2024-01-18 16:45:00',
            messageCount: 0,
            contactCount: 0,
            size: 0
          }
        ],
        selectedDataSource: null,
        syncStatus: 'idle',
        syncProgress: 0,

        // 系统操作
        setOnlineStatus: (isOnline) => set({ isOnline }),
        
        updateLastSync: (timestamp) => set({ lastSync: timestamp }),
        
        addNotification: (notification) => set((state) => ({
          notifications: [
            {
              ...notification,
              id: Date.now().toString(),
              timestamp: new Date().toISOString(),
              read: false
            },
            ...state.notifications
          ]
        })),
        
        markNotificationAsRead: (id) => set((state) => ({
          notifications: state.notifications.map(n => 
            n.id === id ? { ...n, read: true } : n
          )
        })),
        
        clearNotifications: () => set({ notifications: [] }),
        
        setTheme: (theme) => set({ theme }),

        // 数据源操作
        addDataSource: (dataSource) => set((state) => ({
          dataSources: [
            ...state.dataSources,
            {
              ...dataSource,
              id: Date.now().toString()
            }
          ]
        })),
        
        updateDataSource: (id, updates) => set((state) => ({
          dataSources: state.dataSources.map(ds => 
            ds.id === id ? { ...ds, ...updates } : ds
          )
        })),
        
        removeDataSource: (id) => set((state) => ({
          dataSources: state.dataSources.filter(ds => ds.id !== id),
          selectedDataSource: state.selectedDataSource === id ? null : state.selectedDataSource
        })),
        
        setSelectedDataSource: (id) => set({ selectedDataSource: id }),
        
        setSyncStatus: (status) => set({ syncStatus: status }),
        
        setSyncProgress: (progress) => set({ syncProgress: progress }),

        // 计算属性
        getStatistics: () => {
          const state = get();
          return {
            totalDataSources: state.dataSources.length,
            connectedDataSources: state.dataSources.filter(ds => ds.status === 'connected').length,
            totalMessages: state.dataSources.reduce((sum, ds) => sum + ds.messageCount, 0),
            totalContacts: state.dataSources.reduce((sum, ds) => sum + ds.contactCount, 0),
            totalSize: state.dataSources.reduce((sum, ds) => sum + ds.size, 0)
          };
        },
        
        getUnreadNotifications: () => {
          const state = get();
          return state.notifications.filter(n => !n.read);
        }
      }),
      {
        name: 'mwxdump-app-store',
        partialize: (state) => ({
          theme: state.theme,
          dataSources: state.dataSources,
          selectedDataSource: state.selectedDataSource
        })
      }
    ),
    {
      name: 'mwxdump-app-store'
    }
  )
);

// 导出类型
export type { AppState, DataSource, Notification, Statistics };