#![allow
(
    static_mut_refs,
    unused_assignments,
    unused_mut,
    unused_unsafe,
    unused_variables,
)]
pub mod api
{
    use std::arch::{ * };
   /*
    Monotonic Functions Grokked From FASM
    skip_match_ok     | add	esi, eax */
    pub unsafe fn add_from_the_to_the_source( mut rcx:usize, rdx:usize, r8:usize, r9:usize ) -> ( usize, usize, usize, usize )
    {
        unsafe
        {
            asm!
            (
                r#""#,
                inlateout("esi") rcx=>rcx,
            );

            (rcx, rdx, r8, r9)   
        }
    }
}

#[unsafe(no_mangle)] pub unsafe fn memory_option( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn definition_option( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn display_string( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn display_number( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn display_character( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn display_characters( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn display_user_messages( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn display_error_message( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn display_buffer( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
} 


#[unsafe(no_mangle)] pub unsafe fn get_option_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn convert_definition_option( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn initialize( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn high_break( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn exit_program( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn get_environment_variable( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn end_of_variable_name( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn preprocess( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn parse( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn assemble( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn format( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn collect_path( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn adapt_path( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn open_file( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn open_binary_file( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn create_file( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn open_offset( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn read_file( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn read_environment_variable( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn write_file( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn close_file( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn fatal_error( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn dump_preprocessed_source( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn prepare_preprocessed_source( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn restore_preprocessed_source( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn copy_asciiz( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn write_quoted_symbol_name( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn symbol_out_of_scope( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn setup_dump_header( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn skip_preprocessed_line( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn skip_preprocessed_string( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn skip_preprocessed_line_content( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn get_preprocessor_symbol( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn add_preprocessor_symbol( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn preprocess_file( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn use_postponed_macro( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn convert_line( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn preprocess_line( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn lowercase( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn get_directive( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn get_directive_handler_base( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn process_fix_constants( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn process_macro_operators( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn move_data( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn replace_symbolic_constant( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn process_equ_constants( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn calculate_hash( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn skip_macro_argument_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn process_symbolic_constants( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn get_current_line_from_file( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn store_number_symbol( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn get_macro_argument( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn add_macro_symbol( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn finish_macro_argument( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn skip_foreign_line( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn process_macro( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn precalculate_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn convert_expression( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn calculate_expression( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn exact_match( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn skip_match_element( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn match_elements( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn skip_foreign_symbol( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn get_macro_symbol( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn close_macro_block( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn find_macro_symbol_leaf( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}



#[unsafe(no_mangle)] pub unsafe fn skip_fp_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn expand_path( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn get_include_directory( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn get_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn get_data_directive( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn parse_line_contents( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn identify_label( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn get_label_id( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn parse_argument( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn preevaluate_logical_expression( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn get_operator( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn get_symbol( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn parse_address( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn parse_expression( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn allocate_label( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn get_fp_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn fp_optimize( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn fp_mul( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn fp_add( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn fp_div( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn div_64( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn convert_number( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn get_number( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn change_expression( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn preevaluate_logical_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn skip_logical_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn negation_skipped( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn skip_symbol( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
} 

#[unsafe(no_mangle)] pub unsafe fn preevaluate_embedded_logical_expression( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn init_addressing_space( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn assemble_line( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn close_pass( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn error_handler( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn make_label( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn recoverable_overflow( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}



#[unsafe(no_mangle)] pub unsafe fn invalid_count_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn get_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn numeric_name( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn initialize_label( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn invalid_logical_expression( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn instruction_handler( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn get_qword_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn close_virtual_addressing_space( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn get_address_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn finish_label_symbol( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn get_data_point( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}



#[unsafe(no_mangle)] pub unsafe fn div_done( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn store_label_reference( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn mark_mz_relocation( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn recoverable_invalid_address( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn calculate_relative_offset( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}



#[unsafe(no_mangle)] pub unsafe fn fp_qword_small_shift( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn convert_fp_word( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn convert_fp_dword( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn get_string_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn get_byte_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn get_count_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn continue_line( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn allocate_virtual_structure_data( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn allocate_structure_data( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn find_structure_data( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn remove_structure_data( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn find_end_repeat( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn find_structure_end( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn calculate_logical_expression( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn find_end_while( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn find_else( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn find_end_if( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn skip_if_block( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn define_data( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn get_word_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn get_dword_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn check_dword_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn get_pword_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn labels_table_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn write_code( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn mark_relocation( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn undefined_data( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}



#[unsafe(no_mangle)] pub unsafe fn range_exceeded( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn recoverable_misuse( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn make_timestamp( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn add_register( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn sub_register( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn get_byte_scale( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn calculate_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}



#[unsafe(no_mangle)] pub unsafe fn take_bnd_register( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}



#[unsafe(no_mangle)] pub unsafe fn get_bnd_register( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn get_address_component( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn take_bnd_mib( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn cet_size_check( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn take_register( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn get_size_operator( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}



#[unsafe(no_mangle)] pub unsafe fn address_sizes_do_not_match( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn check_rip_relative_address( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn get_address_register( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn get_logical_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn get_value_for_comparison( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn calculate_embedded_logical_expression( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn store_segment_prefix( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}



#[unsafe(no_mangle)] pub unsafe fn int_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn take_byte_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn convert_register( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn operand_autodetect( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn store_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn operand_64bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn get_simm32( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn bad_params( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn invalid_option_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn bad_definition_option( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn file_error( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn preprocessing_finished( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
} 

#[unsafe(no_mangle)] pub unsafe fn line_preprocessed( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn argument_value_end( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn movsw_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn no_more_macro_operators( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn argument_value_length_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn skip_match_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn cannot_match( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn block_closed( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn no_such_macro_symbol( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn local_label_name( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn symbol_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn decorator_symbol( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn no_simple_operator( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn anonymous_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn get_current_offset_id( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn get_counter_id( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn get_timestamp_id( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn get_org_origin_id( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn get_file_offset_id( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn get_actual_file_offset_id( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn expression_converted( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn preevaluated_expression_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn number_done( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn fp_zero( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn logical_value_skipped( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn expression_negation_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn new_label( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn line_assembled( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn no_such_structure( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn else_found( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn error_line( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn undefined_data_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn word_string_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn div_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn zero_count( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn truncated_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn skip_done( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn address64_required( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn recoverable_unknown_size( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn check_mov_address64( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn address64_simm32( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn no_short_jump( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn short_jump( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn bnd_size_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn avx_reg_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn avx_mem_size_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}



#[unsafe(no_mangle)] pub unsafe fn operand_32or64( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn no_vex_source_register( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}



#[unsafe(no_mangle)] pub unsafe fn simple_address_8bit_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn simple_address( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn address_immediate( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn store_address_64bit_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn addressing_16bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn address_16bit_prefix( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn address_32bit_prefix( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn store_instruction_with_imm8( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn store_instruction_with_imm16( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn store_instruction_with_imm32( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn take_avx_rm( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn avx_mem_broadcast_check( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn take_imm4_if_needed( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn take_avx512_mask( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn take_avx512_rounding( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn avx_movq_xmmreg_xmmreg_opcode( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn kmov_w_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}



#[unsafe(no_mangle)] pub unsafe fn take_mask_register( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn setup_kmov_prefix( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn setup_f3_0f_38( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn vex_f2( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn vex_f3( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn vex_66( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn displacement_compression_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn fixup_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}


#[unsafe(no_mangle)] pub unsafe fn string_data_copied( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn finish_pe_relocations( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn close_coff_section( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn relative_coff_64bit_relocation( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn close_coff( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn prepare_default_section( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn default_section_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn section_index_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn elf_segment_position_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn elf64_segment_position_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn store_supplemental_code( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
} 

#[unsafe(no_mangle)] pub unsafe fn preprocessor_symbol_found( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn symbol_characters_matched( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn compare_symbols( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}



#[unsafe(no_mangle)] pub unsafe fn elements_mismatch( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn macro_symbol_not_found( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn qword_number( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn word_number( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn wrongly_structured_logical_expression( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn invalid_logical_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn source_end( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn dword_positive( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn pword_positive( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn vmread_long( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}



#[unsafe(no_mangle)] pub unsafe fn make_pe_fixups( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn make_fixups( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn skip_resource_name( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn skip_unicode_string( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn skip_quoted_string_in_pattern( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn cet_dword( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}



#[unsafe(no_mangle)] pub unsafe fn setup_avx_66_supplemental( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn convert_avx_shift_opcode( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn avx512_pmov_common( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn avx_cvt_q_check_size( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn avx_cvt_q_size_not_specified( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn setup_bextr_imm_opcode( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn displacement_compressed( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn output_written( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}



#[unsafe(no_mangle)] pub unsafe fn get_jump_operator( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn get_address( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn get_address_prefixes( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn operand_16bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn no_address64( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn value_qword( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
} 

#[unsafe(no_mangle)] pub unsafe fn data_offset_from_virtual( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn leave_logical_value_intact( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}



#[unsafe(no_mangle)] pub unsafe fn no_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn label_found( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn register_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn hex_number_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn fp_optimize_done( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn skip_block( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn word_data_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn ret_instruction_only64( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn simple_ret( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn operand_32bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn default_name( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn skip_match_symbol( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn match_verbatim( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn anonymous_label_name( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn operator_found( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn get_byte( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}



#[unsafe(no_mangle)] pub unsafe fn add_in_second_slot( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn create_in_first_slot( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn byte_positive( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn word_positive( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn logical_loop( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn return_false_or_true( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn preevaluated_true( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn simm32_range_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn mov_reg_imm_prefix_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn ret_imm_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn get_reg_reg( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}


#[unsafe(no_mangle)] pub unsafe fn store_mov_reg_imm_code( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn enter_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn ret_instruction_32bit_except64( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn ret_instruction_16bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn ret_instruction_64bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn ret_imm( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn retf_instruction_64bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

complex_dword

#[unsafe(no_mangle)] pub unsafe fn get_pword( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn get_pword( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn complex_pword( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn get_qword( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn large_shift( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn fp_zero_tword( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn complex_tword( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn current_path_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn expression_calculated( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn get_reg_mem( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn calculate_jump_offset( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn make_mmx_prefix( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}




#[unsafe(no_mangle)] pub unsafe fn check_for_short_jump( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn forced_short( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn jump_out_of_range( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn loop_counter_size( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn append_imm8( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn get_mmx_source_register( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn take_additional_xmm0( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn setup_66_0f_3a( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn setup_66_0f_38( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn vmread_check_size( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn no_size_operator( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn store_segment_prefix_if_necessary( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn segment_prefix_386( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn store_instruction_code( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn store_nomem_instruction( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn address_bp( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn address_8bit_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn address( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn sib_address_8bit_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn sib_address( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn address_64bit_relocation_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn default_stub( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn binary_stub( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn section_flag( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn finish_section( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn fixup_32bit( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn generate_pe_data( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn current_address_label_3_characters( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn mark_undefined_data( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn create_in_second_slot( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn plt_relative_offset( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn next_param( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn option_value_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn definition_value_end( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn mmap_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn end_of_variable_name( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn references_dump_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn prepare_next_preprocessed_line( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn restore_next_preprocessed_line( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn file_end( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn line_end( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn lower_case( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn directive_handler( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn not_preprocessor_symbol( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn preprocessor_symbol_not_found( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn fnv1a_hash( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn reuse_symbol_entry( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn extend_hashes_tree( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn file_opened( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn check_qword_value( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn nothing_to_skip( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn clear_address_size( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn address_32bit_relocation_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}


#[unsafe(no_mangle)] pub unsafe fn get_imm32( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn prepare_lwpins( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn take_tile_register( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
} 

#[unsafe(no_mangle)] pub unsafe fn tileloadd_setup( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn store_vex_instruction_code( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn get_vex_lpp_bits( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn store_vex_0f_instruction_code( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn check_vex( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn store_xop_instruction_code( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn evex_rounding( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn evex_b_ok( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn no_directive( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn compare_symbol_types( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn store_symbol_name( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn add_string( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn skip_other_section( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn store_section_index( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn store_elf_machine_word( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn setup_elf_exe_labels_type( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn elf_segment_position_move_and_align( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn close_elf_segment( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn close_elf64_segment( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn elf64_segment_position_move_and_align( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn close_elf_exe( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

#[unsafe(no_mangle)] pub unsafe fn close_elf64_exe( mut rcx:usize, mut rdx:usize, mut r8:usize, mut r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        return ( rcx, rdx, r8, r9 )
    }
}

pub mod env
{
    pub use std::env::{ * };
}

pub mod mem
{
    pub use std::mem::{ * };
}

pub static mut ARGUMENTS:Vec<&'static str> = Vec::new();

pub unsafe fn flat( rcx:usize, rdx:usize, r8:usize, r9:usize ) -> ( usize, usize, usize, usize )
{
    unsafe
    {
        let ( rcx, rdx, r8, mut r9 ) = api::add_from_the_to_the_source( rcx+5, rdx, r8, r9 );
        println!( r#"{:?}"#, rcx );
        r9 = 5;
        ( rcx, rdx, r8, r9 )
    }
}

pub unsafe fn domain( from:() )
{
    unsafe
    {
        if ARGUMENTS.len() > 1 
        {
            for argument in ARGUMENTS.clone()
            {
                println!( r#"{:?}"#, argument );
            }
        }

        let ( rcx, rdx, r8, r9 ) = flat( 0, 0, 0, 0 );
    }
}

pub fn main()
{
    unsafe
    {
        
        let mut arguments = env::args().collect::<Vec<String>>();
        
        if arguments.len() > 1
        {   
            for argument in arguments.clone()
            {
                let arg = argument.clone();
                ARGUMENTS.push( mem::transmute( arg.as_str() ) );
            }
        }

        domain( arguments = vec![] );
    }
}
/*
3450 */
