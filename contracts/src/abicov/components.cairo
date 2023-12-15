//! A contract with components.
#[starknet::interface]
trait ISimple<TContractState> {
    fn read_data(self: @TContractState) -> felt252;
    fn write_data(ref self: TContractState, data: felt252);
}

#[starknet::component]
mod simple_component {
    #[storage]
    struct Storage {
        data: felt252,
    }

    #[derive(Drop, Serde)]
    struct MyStruct {
        a: felt252,
        b: felt252,
    }

    #[event]
    #[derive(Drop, starknet::Event)]
    enum Event {
        Written: Written
    }

    #[derive(Drop, starknet::Event)]
    struct Written {
        before: felt252,
        after: felt252,
    }

    #[embeddable_as(Simple)]
    impl SimpleImpl<
        TContractState, +HasComponent<TContractState>
    > of super::ISimple<ComponentState<TContractState>> {
        fn read_data(self: @ComponentState<TContractState>) -> felt252 {
            self.data.read()
        }

        fn write_data(ref self: ComponentState<TContractState>, data: felt252) {
            let before = self.data.read();
            self.data.write(data);
            self.emit(Written { before, after: data });
        }
    }
}

#[starknet::component]
mod simple_component_other {
    #[storage]
    struct Storage {
        data: felt252,
    }

    #[derive(Drop, Serde)]
    struct MyStruct {
        data: u256,
    }

    #[event]
    #[derive(Drop, starknet::Event)]
    enum Event {
        Written: Written
    }

    #[derive(Drop, starknet::Event)]
    struct Written {
        data: felt252,
    }

    #[embeddable_as(SimpleOther)]
    impl SimpleImpl<
        TContractState, +HasComponent<TContractState>
    > of super::ISimple<ComponentState<TContractState>> {
        fn read_data(self: @ComponentState<TContractState>) -> felt252 {
            self.data.read()
        }

        fn write_data(ref self: ComponentState<TContractState>, data: felt252) {
            self.data.write(data);
            self.emit(Written { data });
        }
    }
}

#[starknet::contract]
mod components_contract {
    use super::simple_component;
    use super::simple_component_other;

    component!(path: simple_component, storage: simple, event: SimpleEvent);
    component!(path: simple_component_other, storage: simple_other, event: SimpleEventOther);

    #[abi(embed_v0)]
    impl SimpleImpl = simple_component::Simple<ContractState>;
    impl SimpleOtherImpl = simple_component_other::SimpleOther<ContractState>;

    #[storage]
    struct Storage {
        value: felt252,
        #[substorage(v0)]
        simple: simple_component::Storage,
        #[substorage(v0)]
        simple_other: simple_component_other::Storage,
    }

    #[event]
    #[derive(Drop, starknet::Event)]
    enum Event {
        OutterEvent: OutterEvent,
        // With flat, only the selector `Written` is set in keys.
        #[flat]
        SimpleEvent: simple_component::Event,
        // Without flat, the first selector in the keys is `SimpleEventOther`, and
        // the second is `Written`.
        SimpleEventOther: simple_component_other::Event,
    }

    #[derive(Drop, starknet::Event)]
    struct OutterEvent {}

    #[external(v0)]
    fn simple(ref self: ContractState) {
        self.simple.write_data(0xaa);
        self.value.write(0xff);
    }

    #[external(v0)]
    fn simple_other(ref self: ContractState) {
        self.simple_other.write_data(0xaa);
        self.value.write(0xee);
    }

    #[external(v0)]
    fn array_struct_simple(ref self: ContractState) -> Span<simple_component::MyStruct> {
        array![].span()
    }

    #[external(v0)]
    fn array_struct_simple_other(
        ref self: ContractState
    ) -> Span<simple_component_other::MyStruct> {
        array![].span()
    }

    #[external(v0)]
    fn tuple_events(
        ref self: ContractState
    ) -> (simple_component::MyStruct, simple_component_other::MyStruct) {
        (
            simple_component::MyStruct { a: 1, b: 2, },
            simple_component_other::MyStruct { data: 'other', },
        )
    }
}
