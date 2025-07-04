use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token};
use std::mem::size_of;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");
// Post and comment text length
const TEXT_LENGTH: usize = 1024;
// Username length
const USER_NAME_LENGTH: usize = 100;
// User profile imaage url length
const USER_URL_LENGTH: usize = 255;

#[program]
pub mod facebook_clone {
    use super::*;
    /// method defination
    /// Create state to save the post counts
    /// There is only one state/constructor in the program
    /// This account should be initialized before post
    pub fn create_state(ctx: Context<CreateState>) -> Result<()> {
        // Get state from context
        let state=&mut ctx.accounts.state;
        // Save authority to state
        state.authority=ctx.accounts.authority.key();
        // Set post count as 0 when initializing
        state.post_count=0;
        Ok(())
    }

    /// Create post
    /// @param text:        text of post
    /// @param poster_name: name of post creator
    /// @param poster_url:  url of post creator avatar
    pub fn create_post(
        ctx: Context<CreatePost>,
        text: String,
        post_author: String,
        post_image_url: String,
    ) -> Result<()> {
        // Get State
        let state=&mut ctx.accounts.state;

        // Get post
        let post =&mut ctx.accounts.post;

        // Set authority
        post.authority=ctx.accounts.authority.key();

        // set text
        post.text=text;

        // set creator name
        post.post_author=post_author;

        // set post image url
        post.post_image_url=post_image_url;

         // set comment counts for post initially zero
        post.comments_count=0;

        // Set post index as state's post count
        post.index=state.post_count;

        // set post time 
        post.post_time=ctx.accounts.clock.unix_timestamp;

        // Increase state's post count by 1
        state.post_count += 1;
        Ok(())
    }

    /// Create comment for post
    /// @param text:            text of comment
    /// @param commenter_name:  name of comment creator
    /// @param commenter_url:   url of comment creator avatar
    pub fn create_comment(
        ctx: Context<CreateComment>,
        text: String,
        commenter_name: String,
        commenter_url: String,
    ) -> Result<()> {
        // Get post
        let post =&mut ctx.accounts.post;
        
        // Get comment
        let comment=&mut ctx.accounts.comment;

        // Set authority to comment
        comment.authority=ctx.accounts.authority.key();

        // Set comment text
        comment.text=text;

        // set commenter_name
        comment.commenter_name=commenter_name;

        // set commenter_url
        comment.commenter_url=commenter_url;

        // Set comment index to post's comment count
        comment.index=post.comments_count;

        // Set post time
        comment.comment_time=ctx.accounts.clock.unix_timestamp;

       // Increase post's comment count by 1
        post.comments_count+=1;
        Ok(())

    }
}

/// validate incoming accounts here
/// Contexts(method declaration via struct) : state
/// CreateState context
#[derive(Accounts)]
pub struct CreateState<'info> {
    // Authenticating state account
    #[account(
        init,
        seeds=[b"state".as_ref()],
        bump,
        payer=authority,
        space=size_of::<StateAccount>()+8
    )]
    pub state:Account<'info,StateAccount>,

    // Authority (this is signer who paid transaction fee)
    #[account(mut)]
    pub authority:Signer<'info>,

    /// System program
    /// /// CHECK: This is not dangerous because we don't read or write from this account
    pub system_program: UncheckedAccount<'info>,

    // TOKEN Program
    #[account(
        constraint=token_program.key == &token::ID
    )]
    pub token_program:Program<'info,Token>,
}


// validate incoming accounts here
// context(method declaration via struct) : create post
/// CreatePost context
#[derive(Accounts)]
pub struct CreatePost<'info>{
    // Authenticate state account
    #[account(mut, seeds = [b"state".as_ref()], bump)]
    pub state: Account<'info, StateAccount>,

    // Authenticate post account
    #[account(
        init,
        // Post account use string "post" and index of post as seeds
        seeds=[b"post".as_ref(),state.post_count.to_be_bytes().as_ref()],
        bump,
        payer=authority,
        space=size_of::<PostAccount>() + TEXT_LENGTH + USER_URL_LENGTH + USER_NAME_LENGTH
    )]
    pub post:Account<'info,PostAccount>,
    /// Authority (this is signer who paid transaction fee)
    #[account(mut)]
    pub authority:Signer<'info>,
    // system program
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub system_program:UncheckedAccount<'info>,
    // TOKEN Program  
    #[account(constraint=token_program.key == &token::ID)]
    pub token_program:Program<'info,Token>,
    // clock to save time
    pub clock: Sysvar<'info, Clock>,

}

// validate incoming accounts here
// context : create comments
/// CreatePost context
#[derive(Accounts)]
pub struct CreateComment<'info>{
    // Authenticate post account
    #[account(mut, seeds = [b"post".as_ref(), post.index.to_be_bytes().as_ref()], bump)]
    pub post:Account<'info,PostAccount>,

    // Authenticate comment account
    #[account(
        init,
        // Post account use string "comment", index of post and index of comment per post as seeds
        seeds = [b"comment".as_ref(), post.index.to_be_bytes().as_ref(), post.comments_count.to_be_bytes().as_ref()],
        bump,
        payer=authority,
        space=size_of::<CommentAccount>()+TEXT_LENGTH+USER_URL_LENGTH+USER_NAME_LENGTH
    )]
    pub comment:Account<'info,CommentAccount>,

    // Authority (this is signer who paid transaction fee)
    #[account(mut)]
    pub authority: Signer<'info>,

    // program system
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub system_program:UncheckedAccount<'info>,

    // TOKEN Program
    pub token_program:Program<'info,Token>,

    // clock to save time
    pub clock:Sysvar<'info,Clock>,

}

// account(storage) : state
// Account is generic over T. This T is a type you can create yourself to store data.
#[account]
pub struct StateAccount {
    // Signer address
    pub authority:Pubkey,

    // post count
    pub post_count:u64,
}

// account(storage) :  post account
#[account]
pub struct PostAccount{
    // Signer address
    pub authority:Pubkey,

    // text field for post
    pub text:String,

    // post creator
    pub post_author:String,

    // post image url
    pub post_image_url:String,

    // comment counts for post
    pub comments_count:u64,

    // post index
    pub index:u64,

    // posting time
    pub post_time:i64,
}

#[account]
pub struct CommentAccount{
    // Signer address
    pub authority:Pubkey,

    // comment text
    pub text:String,

    // commenter_name
    pub commenter_name:String,

    // commenter_url
    pub commenter_url:String,

    // Comment index
    pub index: u64,

    // comment time
    comment_time:i64,

}
