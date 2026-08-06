// Handwritten operational behavior for the authority-verified ordinary Mirror Interface.
//
// The strict bootstrap projection owns every structural type below. This file
// supplies only current-stage behavior: structural traits over the ordinary
// producer's shared representation, readable Dotos roles, and the allocated
// Signal frame boundary.

use rkyv::{
    Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize,
    rancor::Source as _,
};
use signal_standard::schema::lib::{WireShape, WireShapeError, WireValue};

fn one_field(mut fields: Vec<WireValue>) -> Result<WireValue, WireShapeError> {
    if fields.len() != 1 {
        return Err(WireShapeError);
    }
    Ok(fields.pop().expect("one field checked"))
}

macro_rules! wire_traits {
    ($name:ident) => {
        impl Clone for $name { fn clone(&self) -> Self { Self::from_wire(self.to_wire()).expect("a projected value revalidates") } }
        impl std::fmt::Debug for $name { fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { self.to_wire().fmt(formatter) } }
        impl PartialEq for $name { fn eq(&self, other: &Self) -> bool { self.to_wire() == other.to_wire() } }
        impl Eq for $name {}
    };
}
macro_rules! wire_external_newtype {
    ($name:ident, $inner:ty) => {
        impl WireShape for $name {
            fn to_wire(&self) -> WireValue { self.payload().to_wire() }
            fn from_wire(value: WireValue) -> Result<Self, WireShapeError> { Ok(Self::new(<$inner as WireShape>::from_wire(value)?)) }
        }
        wire_traits!($name);
        #[cfg(feature = "dotos-text")]
        impl dotos::DotosEncode for $name {
            fn to_dotos(&self) -> std::string::String {
                dotos::DotosEncode::to_dotos(self.payload())
            }
        }
        #[cfg(feature = "dotos-text")]
        impl dotos::DotosDecode for $name {
            fn from_dotos_block(block: &dotos::Block) -> Result<Self, dotos::DotosDecodeError> {
                <$inner as dotos::DotosDecode>::from_dotos_block(block).map(Self::new)
            }
        }
    };
}
macro_rules! wire_newtype {
    ($name:ident, $inner:ty) => {
        impl $name {
            pub fn new(payload: $inner) -> Self {
                Self(payload)
            }
            pub fn payload(&self) -> &$inner {
                &self.0
            }
            pub fn into_payload(self) -> $inner {
                self.0
            }
        }
        impl WireShape for $name {
            fn to_wire(&self) -> WireValue {
                self.0.to_wire()
            }
            fn from_wire(value: WireValue) -> Result<Self, WireShapeError> {
                Ok(Self(<$inner as WireShape>::from_wire(value)?))
            }
        }
        wire_traits!($name);
        #[cfg(feature = "dotos-text")]
        impl dotos::DotosEncode for $name {
            fn to_dotos(&self) -> std::string::String {
                dotos::DotosEncode::to_dotos(&self.0)
            }
        }
        #[cfg(feature = "dotos-text")]
        impl dotos::DotosDecode for $name {
            fn from_dotos_block(block: &dotos::Block) -> Result<Self, dotos::DotosDecodeError> {
                <$inner as dotos::DotosDecode>::from_dotos_block(block).map(Self)
            }
        }
    };
}
macro_rules! wire_struct {
    ($name:ident { $($field:ident: $field_type:ty),* $(,)? }) => {
        impl WireShape for $name {
            fn to_wire(&self) -> WireValue { WireValue::Product(vec![$(self.$field.to_wire()),*]) }
            fn from_wire(value: WireValue) -> Result<Self, WireShapeError> {
                let WireValue::Product(fields) = value else { return Err(WireShapeError) };
                let mut fields = fields.into_iter();
                let result = Self { $($field: <$field_type as WireShape>::from_wire(fields.next().ok_or(WireShapeError)?)?),* };
                if fields.next().is_some() { return Err(WireShapeError); }
                Ok(result)
            }
        }
        wire_traits!($name);
        #[cfg(feature = "dotos-text")]
        impl dotos::DotosEncode for $name {
            fn to_dotos(&self) -> std::string::String {
                dotos::Delimiter::Parenthesis.wrap([
                    $(dotos::DotosEncode::to_dotos(&self.$field)),*
                ])
            }
        }
        #[cfg(feature = "dotos-text")]
        impl dotos::DotosDecode for $name {
            fn from_dotos_block(block: &dotos::Block) -> Result<Self, dotos::DotosDecodeError> {
                let body = dotos::DotosBody::from_delimited(
                    block,
                    dotos::Delimiter::Parenthesis,
                    stringify!($name),
                )?;
                let expected = 0usize $(+ {
                    let _ = stringify!($field);
                    1usize
                })*;
                #[allow(unused_mut, unused_variables)]
                let mut fields = body.expect_fields(stringify!($name), expected)?.iter();
                Ok(Self {
                    $($field: <$field_type as dotos::DotosDecode>::from_dotos_block(
                        fields.next().expect("field count checked"),
                    )?),*
                })
            }
        }
    };
}
macro_rules! wire_enum {
    ($name:ident {
        unit { $($unit_ordinal:literal => $unit:ident : $unit_visible:literal),* $(,)? }
        unary { $($unary_ordinal:literal => $unary:ident($payload:ty) : $unary_visible:literal),* $(,)? }
    }) => {
        impl WireShape for $name {
            fn to_wire(&self) -> WireValue {
                match self {
                    $(Self::$unit => WireValue::Variant { ordinal: $unit_ordinal, fields: Vec::new() },)*
                    $(Self::$unary(payload) => WireValue::Variant { ordinal: $unary_ordinal, fields: vec![payload.to_wire()] },)*
                }
            }
            fn from_wire(value: WireValue) -> Result<Self, WireShapeError> {
                let WireValue::Variant { ordinal, fields } = value else { return Err(WireShapeError) };
                match ordinal {
                    $($unit_ordinal if fields.is_empty() => Ok(Self::$unit),)*
                    $($unary_ordinal => Ok(Self::$unary(<$payload as WireShape>::from_wire(one_field(fields)?)?)),)*
                    _ => Err(WireShapeError),
                }
            }
        }
        wire_traits!($name);
        #[cfg(feature = "dotos-text")]
        impl dotos::DotosEncode for $name {
            fn to_dotos(&self) -> std::string::String {
                match self {
                    $(Self::$unit => $unit_visible.to_owned(),)*
                    $(Self::$unary(payload) => format!(
                        "{}.{}",
                        $unary_visible,
                        dotos::DotosEncode::to_dotos(payload),
                    ),)*
                }
            }
        }
        #[cfg(feature = "dotos-text")]
        impl dotos::DotosDecode for $name {
            fn from_dotos_block(block: &dotos::Block) -> Result<Self, dotos::DotosDecodeError> {
                if let Some(variant) = block.demote_to_string() {
                    return match variant {
                        $($unit_visible => Ok(Self::$unit),)*
                        _ => Err(dotos::DotosDecodeError::UnknownVariant {
                            enum_name: stringify!($name),
                            variant: variant.to_owned(),
                        }),
                    };
                }
                let (head, payload) = block.as_application().ok_or(
                    dotos::DotosDecodeError::ExpectedAtom { type_name: stringify!($name) },
                )?;
                let _ = &payload;
                let variant = head.demote_to_string().ok_or(
                    dotos::DotosDecodeError::ExpectedAtom { type_name: stringify!($name) },
                )?;
                match variant {
                    $($unary_visible => Ok(Self::$unary(
                        <$payload as dotos::DotosDecode>::from_dotos_block(payload)?,
                    )),)*
                    _ => Err(dotos::DotosDecodeError::UnknownVariant {
                        enum_name: stringify!($name),
                        variant: variant.to_owned(),
                    }),
                }
            }
        }
    };
}
wire_enum!(z2VU1V { unit {  } unary { 0 => z2VPpf(z2VaGZ) : "Configure", 1 => z2VWRp(z2VapR) : "Inspect" } });
wire_enum!(z2VTar { unit {  } unary { 0 => z2VW97(z2Vaeg) : "RequestUnimplemented", 1 => z2VT9W(z2VM6r) : "PolicySnapshot", 2 => z2VRex(z2VMrs) : "Configured", 3 => z2VTDv(z2VWsK) : "ConfigurationRejected" } });
wire_newtype!(z2VWhN, z2VaGZ);
wire_newtype!(z2VbJR, z2VapR);
wire_enum!(z2VdFT { unit { 0 => z2VNyX : "ObserveOnly", 1 => z2VQaM : "IssueOrders", 2 => z2VLfW : "ProposeOrders" } unary {  } });
wire_enum!(z2VSX6 { unit { 0 => z2VT4y : "Decide", 1 => z2VdkJ : "Recommend", 2 => z2VTjN : "RecordOnly" } unary {  } });
wire_enum!(z2VYng { unit { 0 => z2VZBx : "Disabled", 1 => z2VQsW : "SummaryOnly", 2 => z2VMUU : "FullRecord" } unary {  } });
wire_struct!(z2VaGZ { field_0: z2VdFT, field_1: z2VSX6, field_2: z2VYng });
wire_enum!(z2VKp8 { unit { 0 => z2Vaxk : "Choreography", 1 => z2VUez : "IntentSynchronization", 2 => z2VKoe : "Authority", 3 => z2Vexw : "All" } unary {  } });
wire_newtype!(z2VapR, z2VKp8);
wire_struct!(z2VM6r { field_0: z2VMrs, field_1: z2VaGZ });
wire_enum!(z2VXWc { unit { 0 => z2VaHh : "IntentSynchronizationUnavailable", 1 => z2VagN : "SpiritAuthorityRequired", 2 => z2VZ2n : "PolicyWouldBreakChoreography" } unary {  } });
wire_newtype!(z2VWsK, z2VXWc);
wire_enum!(z2VYqg { unit { 0 => z2VPdd : "Configure", 1 => z2VXQL : "Inspect" } unary {  } });
wire_enum!(z2VXPw { unit { 0 => z2VWnx : "PolicyStoreUnavailable", 1 => z2VMzC : "NotBuiltYet", 2 => z2VUUL : "DependencyNotReady" } unary {  } });
wire_struct!(z2Vaeg { field_0: z2VYqg, field_1: z2VXPw });
wire_external_newtype!(z2VMrs, u64);

macro_rules! archive_root {
    ($root:ident) => {
        impl Archive for $root {
            type Archived = <WireValue as Archive>::Archived;
            type Resolver = <WireValue as Archive>::Resolver;
            fn resolve(&self, resolver: Self::Resolver, out: rkyv::Place<Self::Archived>) {
                self.to_wire().resolve(resolver, out);
            }
        }
        impl<Serializer> RkyvSerialize<Serializer> for $root
        where
            Serializer: rkyv::rancor::Fallible + ?Sized,
            WireValue: RkyvSerialize<Serializer>,
        {
            fn serialize(
                &self,
                serializer: &mut Serializer,
            ) -> Result<Self::Resolver, Serializer::Error> {
                self.to_wire().serialize(serializer)
            }
        }
        impl<Deserializer> RkyvDeserialize<$root, Deserializer>
            for signal_standard::schema::lib::ArchivedWireValue
        where
            Deserializer: rkyv::rancor::Fallible + ?Sized,
            Deserializer::Error: rkyv::rancor::Source,
            signal_standard::schema::lib::ArchivedWireValue:
                RkyvDeserialize<WireValue, Deserializer>,
        {
            fn deserialize(
                &self,
                deserializer: &mut Deserializer,
            ) -> Result<$root, Deserializer::Error> {
                let wire = <signal_standard::schema::lib::ArchivedWireValue as RkyvDeserialize<
                    WireValue,
                    Deserializer,
                >>::deserialize(self, deserializer)?;
                <$root as WireShape>::from_wire(wire).map_err(Deserializer::Error::new)
            }
        }
    };
}
archive_root!(z2VU1V);
archive_root!(z2VTar);
archive_root!(z2VWhN);
archive_root!(z2VbJR);
archive_root!(z2VdFT);
archive_root!(z2VSX6);
archive_root!(z2VYng);
archive_root!(z2VaGZ);
archive_root!(z2VKp8);
archive_root!(z2VapR);
archive_root!(z2VM6r);
archive_root!(z2VXWc);
archive_root!(z2VWsK);
archive_root!(z2VYqg);
archive_root!(z2VXPw);
archive_root!(z2Vaeg);
archive_root!(z2VMrs);


pub enum ContractMarker {}
impl signal_frame::WireContract for ContractMarker {
    const BINDING: signal_frame::ContractBinding = signal_frame::ContractBinding::new(
        match signal_frame::ContractId::try_new(14) { Ok(value) => value, Err(_) => panic!("contract ID is allocated") },
        match signal_frame::WireRevision::try_new(2) { Ok(value) => value, Err(_) => panic!("wire revision is allocated") },
    );
}

#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum SignalFrameError {
    #[error("failed to encode bound signal frame")] FrameEncode,
    #[error("failed to decode bound signal frame")] ArchiveDecode,
    #[error("unexpected signal frame body")] UnexpectedFrameBody,
    #[error("expected one request operation, found {found}")] OperationCount { found: usize },
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)] #[repr(u8)] pub enum InputRoute { Configure, Inspect }
#[derive(Clone, Copy, Debug, PartialEq, Eq)] #[repr(u8)] pub enum OutputRoute { RequestUnimplemented, PolicySnapshot, Configured, ConfigurationRejected }
impl z2VU1V {
 pub fn route(&self)->InputRoute{match self{Self::z2VPpf(_)=>InputRoute::Configure,Self::z2VWRp(_)=>InputRoute::Inspect,}}
 pub fn wire_route(&self)->signal_frame::WireRoute{signal_frame::WireRoute::new(signal_frame::RootCode::new(0),signal_frame::VariantCode::new(self.route() as u8))}
 pub fn into_frame(self,exchange:signal_frame::ExchangeIdentifier)->Frame{let route=self.wire_route();Frame::new(route,FrameBody::Request{exchange,request:signal_frame::Request::from_payload(self)})}
 pub fn encode_request_frame(self,exchange:signal_frame::ExchangeIdentifier)->Result<Vec<u8>,SignalFrameError>{self.into_frame(exchange).encode().map_err(|_|SignalFrameError::FrameEncode)}
}
impl z2VTar {
 pub fn route(&self)->OutputRoute{match self{Self::z2VW97(_)=>OutputRoute::RequestUnimplemented,Self::z2VT9W(_)=>OutputRoute::PolicySnapshot,Self::z2VRex(_)=>OutputRoute::Configured,Self::z2VTDv(_)=>OutputRoute::ConfigurationRejected,}}
 pub fn wire_route(&self)->signal_frame::WireRoute{signal_frame::WireRoute::new(signal_frame::RootCode::new(1),signal_frame::VariantCode::new(self.route() as u8))}
 pub fn into_reply_frame(self,exchange:signal_frame::ExchangeIdentifier)->Frame{let route=self.wire_route();let reply=signal_frame::Reply::committed(signal_frame::NonEmpty::single(signal_frame::SubReply::Ok(self)));Frame::new(route,FrameBody::Reply{exchange,reply})}
 pub fn encode_reply_frame(self,exchange:signal_frame::ExchangeIdentifier)->Result<Vec<u8>,SignalFrameError>{self.into_reply_frame(exchange).encode().map_err(|_|SignalFrameError::FrameEncode)}
}
impl signal_frame::RequestPayload for z2VU1V {}
impl signal_frame::SignalOperationHeads for z2VU1V{const HEADS:&'static[&'static str]=&["Configure","Inspect"];}
impl signal_frame::LogVariant for z2VU1V{fn log_variant(&self)->u64{let route=self.wire_route();u64::from(route.root().value())|(u64::from(route.variant().value())<<8)}}
pub type Frame=signal_frame::BoundExchangeFrame<ContractMarker,z2VU1V,z2VTar>;
pub type FrameBody=signal_frame::ExchangeFrameBody<z2VU1V,z2VTar>;
pub type Request=signal_frame::Request<z2VU1V>;
pub type ReplyEnvelope=signal_frame::Reply<z2VTar>;
pub type RequestBuilder=signal_frame::RequestBuilder<z2VU1V>;
impl ContractMarker{
 pub fn decode_frame(bytes:&[u8])->Result<Frame,SignalFrameError>{Frame::decode(bytes).map_err(|_|SignalFrameError::ArchiveDecode)}
 pub fn decode_single_request(bytes:&[u8])->Result<(signal_frame::ExchangeIdentifier,z2VU1V),SignalFrameError>{match Self::decode_frame(bytes)?.into_body(){FrameBody::Request{exchange,request}=>{let found=request.payloads().len();if found!=1{return Err(SignalFrameError::OperationCount{found});}Ok((exchange,request.payloads.into_head()))},_=>Err(SignalFrameError::UnexpectedFrameBody)}}
}

