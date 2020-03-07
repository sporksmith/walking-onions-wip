# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: snip.proto

import sys
_b=sys.version_info[0]<3 and (lambda x:x) or (lambda x:x.encode('latin1'))
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from google.protobuf import reflection as _reflection
from google.protobuf import symbol_database as _symbol_database
# @@protoc_insertion_point(imports)

_sym_db = _symbol_database.Default()




DESCRIPTOR = _descriptor.FileDescriptor(
  name='snip.proto',
  package='',
  syntax='proto3',
  serialized_options=None,
  serialized_pb=_b('\n\nsnip.proto\"\x95\x02\n\x04Snip\x12\x11\n\ttimestamp\x18\x01 \x01(\x04\x12\x10\n\x08lifetime\x18\x02 \x01(\r\x12\x11\n\tlinkspecs\x18\x03 \x03(\x0c\x12\x11\n\ted_id_key\x18\x04 \x01(\x0c\x12\x10\n\x08ntor_key\x18\x05 \x01(\x0c\x12\x10\n\x08software\x18\x06 \x01(\t\x12!\n\tprotovers\x18\x07 \x03(\x0b\x32\x0e.Snip.ProtoVer\x12\x0e\n\x06\x66\x61mily\x18\x08 \x01(\x0c\x12\n\n\x02\x63\x63\x18\t \x01(\t\x12\x0f\n\x07idxtype\x18\n \x01(\r\x12\x0e\n\x06idxlow\x18\x0b \x01(\x07\x12\x0f\n\x07idxhigh\x18\x0c \x01(\x07\x1a-\n\x08ProtoVer\x12\t\n\x01p\x18\x01 \x01(\r\x12\n\n\x02lo\x18\x02 \x01(\r\x12\n\n\x02hi\x18\x03 \x01(\rb\x06proto3')
)




_SNIP_PROTOVER = _descriptor.Descriptor(
  name='ProtoVer',
  full_name='Snip.ProtoVer',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  fields=[
    _descriptor.FieldDescriptor(
      name='p', full_name='Snip.ProtoVer.p', index=0,
      number=1, type=13, cpp_type=3, label=1,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='lo', full_name='Snip.ProtoVer.lo', index=1,
      number=2, type=13, cpp_type=3, label=1,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='hi', full_name='Snip.ProtoVer.hi', index=2,
      number=3, type=13, cpp_type=3, label=1,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR),
  ],
  extensions=[
  ],
  nested_types=[],
  enum_types=[
  ],
  serialized_options=None,
  is_extendable=False,
  syntax='proto3',
  extension_ranges=[],
  oneofs=[
  ],
  serialized_start=247,
  serialized_end=292,
)

_SNIP = _descriptor.Descriptor(
  name='Snip',
  full_name='Snip',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  fields=[
    _descriptor.FieldDescriptor(
      name='timestamp', full_name='Snip.timestamp', index=0,
      number=1, type=4, cpp_type=4, label=1,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='lifetime', full_name='Snip.lifetime', index=1,
      number=2, type=13, cpp_type=3, label=1,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='linkspecs', full_name='Snip.linkspecs', index=2,
      number=3, type=12, cpp_type=9, label=3,
      has_default_value=False, default_value=[],
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='ed_id_key', full_name='Snip.ed_id_key', index=3,
      number=4, type=12, cpp_type=9, label=1,
      has_default_value=False, default_value=_b(""),
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='ntor_key', full_name='Snip.ntor_key', index=4,
      number=5, type=12, cpp_type=9, label=1,
      has_default_value=False, default_value=_b(""),
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='software', full_name='Snip.software', index=5,
      number=6, type=9, cpp_type=9, label=1,
      has_default_value=False, default_value=_b("").decode('utf-8'),
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='protovers', full_name='Snip.protovers', index=6,
      number=7, type=11, cpp_type=10, label=3,
      has_default_value=False, default_value=[],
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='family', full_name='Snip.family', index=7,
      number=8, type=12, cpp_type=9, label=1,
      has_default_value=False, default_value=_b(""),
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='cc', full_name='Snip.cc', index=8,
      number=9, type=9, cpp_type=9, label=1,
      has_default_value=False, default_value=_b("").decode('utf-8'),
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='idxtype', full_name='Snip.idxtype', index=9,
      number=10, type=13, cpp_type=3, label=1,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='idxlow', full_name='Snip.idxlow', index=10,
      number=11, type=7, cpp_type=3, label=1,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='idxhigh', full_name='Snip.idxhigh', index=11,
      number=12, type=7, cpp_type=3, label=1,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR),
  ],
  extensions=[
  ],
  nested_types=[_SNIP_PROTOVER, ],
  enum_types=[
  ],
  serialized_options=None,
  is_extendable=False,
  syntax='proto3',
  extension_ranges=[],
  oneofs=[
  ],
  serialized_start=15,
  serialized_end=292,
)

_SNIP_PROTOVER.containing_type = _SNIP
_SNIP.fields_by_name['protovers'].message_type = _SNIP_PROTOVER
DESCRIPTOR.message_types_by_name['Snip'] = _SNIP
_sym_db.RegisterFileDescriptor(DESCRIPTOR)

Snip = _reflection.GeneratedProtocolMessageType('Snip', (_message.Message,), dict(

  ProtoVer = _reflection.GeneratedProtocolMessageType('ProtoVer', (_message.Message,), dict(
    DESCRIPTOR = _SNIP_PROTOVER,
    __module__ = 'snip_pb2'
    # @@protoc_insertion_point(class_scope:Snip.ProtoVer)
    ))
  ,
  DESCRIPTOR = _SNIP,
  __module__ = 'snip_pb2'
  # @@protoc_insertion_point(class_scope:Snip)
  ))
_sym_db.RegisterMessage(Snip)
_sym_db.RegisterMessage(Snip.ProtoVer)


# @@protoc_insertion_point(module_scope)
