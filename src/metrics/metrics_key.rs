use crate::metrics::model::MetricsType;
use lazy_static::lazy_static;

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum MetricsKey {
    //config
    ConfigDataSize,
    ConfigListenerSize,
    ConfigSubscriberListenerKeySize,
    ConfigSubscriberListenerValueSize,
    ConfigSubscriberClientSize,
    ConfigSubscriberClientValueSize,
    ConfigIndexTenantSize,
    ConfigIndexConfigSize,
    //naming
    NamingServiceSize,
    NamingInstanceSize,
    NamingSubscriberListenerKeySize,
    NamingSubscriberListenerValueSize,
    NamingSubscriberClientSize,
    NamingSubscriberClientValueSize,
    NamingEmptyServiceSetSize,
    NamingEmptyServiceSetItemSize,
    NamingInstanceMetaSetSize,
    NamingInstanceMetaSetItemSize,
    NamingHealthyTimeoutSetSize,
    NamingHealthyTimeoutSetItemSize,
    NamingUnhealthyTimeoutSetSize,
    NamingUnhealthyTimeoutSetItemSize,
    NamingClientInstanceSetKeySize,
    NamingClientInstanceSetValueSize,
    NamingIndexTenantSize,
    NamingIndexGroupSize,
    NamingIndexServiceSize,
}

lazy_static! {
    /// 用于有序遍历打印信息
    pub static ref ORDER_ALL_KEYS: Vec<MetricsKey> = vec![
        MetricsKey::ConfigDataSize,
        MetricsKey::ConfigListenerSize,
        MetricsKey::ConfigSubscriberListenerKeySize,
        MetricsKey::ConfigSubscriberListenerValueSize,
        MetricsKey::ConfigSubscriberClientSize,
        MetricsKey::ConfigSubscriberClientValueSize,
        MetricsKey::ConfigIndexTenantSize,
        MetricsKey::ConfigIndexConfigSize,
        MetricsKey::NamingServiceSize,
        MetricsKey::NamingInstanceSize,
        MetricsKey::NamingSubscriberListenerKeySize,
        MetricsKey::NamingSubscriberListenerValueSize,
        MetricsKey::NamingSubscriberClientSize,
        MetricsKey::NamingSubscriberClientValueSize,
        MetricsKey::NamingEmptyServiceSetSize,
        MetricsKey::NamingEmptyServiceSetItemSize,
        MetricsKey::NamingInstanceMetaSetSize,
        MetricsKey::NamingInstanceMetaSetItemSize,
        MetricsKey::NamingHealthyTimeoutSetSize,
        MetricsKey::NamingHealthyTimeoutSetItemSize,
        MetricsKey::NamingUnhealthyTimeoutSetSize,
        MetricsKey::NamingUnhealthyTimeoutSetItemSize,
        MetricsKey::NamingClientInstanceSetKeySize,
        MetricsKey::NamingClientInstanceSetValueSize,
        MetricsKey::NamingIndexTenantSize,
        MetricsKey::NamingIndexGroupSize,
        MetricsKey::NamingIndexServiceSize,
    ];
}

impl MetricsKey {
    pub fn get_key(&self) -> &'static str {
        match &self {
            MetricsKey::ConfigDataSize => "ConfigDataSize",
            MetricsKey::ConfigListenerSize => "ConfigListenerSize",
            MetricsKey::ConfigSubscriberListenerKeySize => "ConfigSubscriberListenerKeySize",
            MetricsKey::ConfigSubscriberListenerValueSize => "ConfigSubscriberListenerValueSize",
            MetricsKey::ConfigSubscriberClientSize => "ConfigSubscriberClientSize",
            MetricsKey::ConfigSubscriberClientValueSize => "ConfigSubscriberClientValueSize",
            MetricsKey::ConfigIndexTenantSize => "ConfigIndexTenantSize",
            MetricsKey::ConfigIndexConfigSize => "ConfigIndexConfigSize",
            MetricsKey::NamingServiceSize => "NamingServiceSize",
            MetricsKey::NamingInstanceSize => "NamingInstanceSize",
            MetricsKey::NamingSubscriberListenerKeySize => "NamingSubscriberListenerKeySize",
            MetricsKey::NamingSubscriberListenerValueSize => "NamingSubscriberListenerValueSize",
            MetricsKey::NamingSubscriberClientSize => "NamingSubscriberClientSize",
            MetricsKey::NamingSubscriberClientValueSize => "NamingSubscriberClientValueSize",
            MetricsKey::NamingEmptyServiceSetSize => "NamingEmptyServiceSetSize",
            MetricsKey::NamingEmptyServiceSetItemSize => "NamingEmptyServiceSetItemSize",
            MetricsKey::NamingInstanceMetaSetSize => "NamingInstanceMetaSetSize",
            MetricsKey::NamingInstanceMetaSetItemSize => "NamingInstanceMetaSetItemSize",
            MetricsKey::NamingHealthyTimeoutSetSize => "NamingHealthyTimeoutSetSize",
            MetricsKey::NamingHealthyTimeoutSetItemSize => "NamingHealthyTimeoutSetItemSize",
            MetricsKey::NamingUnhealthyTimeoutSetSize => "NamingUnhealthyTimeoutSetSize",
            MetricsKey::NamingUnhealthyTimeoutSetItemSize => "NamingUnhealthyTimeoutSetItemSize",
            MetricsKey::NamingClientInstanceSetKeySize => "NamingClientInstanceSetKeySize",
            MetricsKey::NamingClientInstanceSetValueSize => "NamingClientInstanceSetValueSize",
            MetricsKey::NamingIndexTenantSize => "NamingIndexTenantSize",
            MetricsKey::NamingIndexGroupSize => "NamingIndexGroupSize",
            MetricsKey::NamingIndexServiceSize => "NamingIndexServiceSize",
        }
    }

    pub fn get_metrics_type(&self) -> MetricsType {
        match &self {
            MetricsKey::ConfigDataSize => MetricsType::Gauge,
            MetricsKey::ConfigListenerSize => MetricsType::Gauge,
            MetricsKey::ConfigSubscriberListenerKeySize => MetricsType::Gauge,
            MetricsKey::ConfigSubscriberListenerValueSize => MetricsType::Gauge,
            MetricsKey::ConfigSubscriberClientSize => MetricsType::Gauge,
            MetricsKey::ConfigSubscriberClientValueSize => MetricsType::Gauge,
            MetricsKey::ConfigIndexTenantSize => MetricsType::Gauge,
            MetricsKey::ConfigIndexConfigSize => MetricsType::Gauge,
            MetricsKey::NamingServiceSize => MetricsType::Gauge,
            MetricsKey::NamingInstanceSize => MetricsType::Gauge,
            MetricsKey::NamingSubscriberListenerKeySize => MetricsType::Gauge,
            MetricsKey::NamingSubscriberListenerValueSize => MetricsType::Gauge,
            MetricsKey::NamingSubscriberClientSize => MetricsType::Gauge,
            MetricsKey::NamingSubscriberClientValueSize => MetricsType::Gauge,
            MetricsKey::NamingEmptyServiceSetSize => MetricsType::Gauge,
            MetricsKey::NamingEmptyServiceSetItemSize => MetricsType::Gauge,
            MetricsKey::NamingInstanceMetaSetSize => MetricsType::Gauge,
            MetricsKey::NamingInstanceMetaSetItemSize => MetricsType::Gauge,
            MetricsKey::NamingHealthyTimeoutSetSize => MetricsType::Gauge,
            MetricsKey::NamingHealthyTimeoutSetItemSize => MetricsType::Gauge,
            MetricsKey::NamingUnhealthyTimeoutSetSize => MetricsType::Gauge,
            MetricsKey::NamingUnhealthyTimeoutSetItemSize => MetricsType::Gauge,
            MetricsKey::NamingClientInstanceSetKeySize => MetricsType::Gauge,
            MetricsKey::NamingClientInstanceSetValueSize => MetricsType::Gauge,
            MetricsKey::NamingIndexTenantSize => MetricsType::Gauge,
            MetricsKey::NamingIndexGroupSize => MetricsType::Gauge,
            MetricsKey::NamingIndexServiceSize => MetricsType::Gauge,
        }
    }
}
